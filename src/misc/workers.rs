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

static POOL: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("tw-worker")
        .build()
        .expect("worker pool")
});

static QUEUE: Mutex<Queue> = Mutex::new(Queue {
    jobs: VecDeque::new(),
    running: 0,
});

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
    QUEUE.lock().unwrap().jobs.push_back(Box::new(job));
    pump();
}

pub fn pump() {
    let mut queue = QUEUE.lock().unwrap();
    while queue.running < config().worker_threads().max(1)
        && let Some(job) = queue.jobs.pop_front()
    {
        queue.running += 1;
        POOL.spawn_blocking(move || {
            let _ = catch_unwind(AssertUnwindSafe(job));
            QUEUE.lock().unwrap().running -= 1;
            pump();
        });
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
