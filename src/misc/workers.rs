use crate::misc::config::config;
use std::{
    collections::VecDeque,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{LazyLock, Mutex},
};
use tokio::runtime::{Builder, Runtime};

type Job = Box<dyn FnOnce() + Send>;

struct Queue {
    jobs: VecDeque<Job>,
    running: usize,
}

impl Queue {
    const fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
            running: 0,
        }
    }
}

#[derive(Clone, Copy)]
enum Lane {
    Long,
    Short,
}

impl Lane {
    fn cap(self) -> usize {
        let threads = config().worker_threads().max(1);
        match self {
            Lane::Long => threads.div_ceil(2),
            Lane::Short => (threads / 2).max(1),
        }
    }
}

static POOL: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("tw-worker")
        .build()
        .expect("worker pool")
});

static QUEUES: Mutex<[Queue; 2]> = Mutex::new([Queue::new(), Queue::new()]);

pub fn release_memory() {
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        unsafe extern "C" {
            fn malloc_trim(pad: usize) -> i32;
        }
        unsafe { malloc_trim(0) };
    }
}

pub fn spawn(job: impl FnOnce() + Send + 'static) {
    push(Lane::Long, Box::new(job));
}

pub fn spawn_short(job: impl FnOnce() + Send + 'static) {
    push(Lane::Short, Box::new(job));
}

fn push(lane: Lane, job: Job) {
    QUEUES.lock().unwrap()[lane as usize].jobs.push_back(job);
    pump();
}

pub fn pump() {
    let mut queues = QUEUES.lock().unwrap();
    for lane in [Lane::Long, Lane::Short] {
        let queue = &mut queues[lane as usize];
        while queue.running < lane.cap()
            && let Some(job) = queue.jobs.pop_front()
        {
            queue.running += 1;
            POOL.spawn_blocking(move || {
                let _ = catch_unwind(AssertUnwindSafe(job));
                QUEUES.lock().unwrap()[lane as usize].running -= 1;
                pump();
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::{Duration, Instant},
    };

    #[test]
    fn jobs_beyond_the_cap_wait_and_panics_free_their_slot() {
        let cap = config().worker_threads();
        let running = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let done = Arc::new(AtomicUsize::new(0));
        spawn(|| panic!("job bug"));
        for _ in 0..cap * 3 {
            let (running, peak, done) = (running.clone(), peak.clone(), done.clone());
            spawn(move || {
                peak.fetch_max(running.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(20));
                running.fetch_sub(1, Ordering::SeqCst);
                done.fetch_add(1, Ordering::SeqCst);
            });
        }
        let start = Instant::now();
        while done.load(Ordering::SeqCst) < cap * 3 {
            assert!(start.elapsed() < Duration::from_secs(10));
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(peak.load(Ordering::SeqCst) <= cap);
    }
}
