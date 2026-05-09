use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
};

#[derive(Debug)]
pub struct BackgroundHandle<P: Clone, T> {
    alive: SetIsAlive,
    progress: GetProgress<P>,
    hndl: JoinHandle<T>,
}

impl<P, T> BackgroundHandle<P, T>
where
    P: Clone,
{
    pub fn cancel(&self) {
        self.alive.set(false);
    }

    pub fn progress(&self) -> P {
        self.progress.get()
    }

    pub fn result(self) -> Option<T> {
        self.hndl.join().unwrap().into()
    }

    pub fn is_running(&self) -> bool {
        self.hndl.is_finished()
    }
}

#[derive(Debug)]
pub struct GetIsAlive {
    is_alive: Arc<AtomicBool>,
}

impl GetIsAlive {
    pub fn get(&self) -> bool {
        self.is_alive.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
struct SetIsAlive {
    is_alive: Arc<AtomicBool>,
}

impl SetIsAlive {
    fn set(&self, val: bool) {
        self.is_alive.store(val, Ordering::Relaxed);
    }
}

fn is_alive() -> (GetIsAlive, SetIsAlive) {
    let is_alive = Arc::new(AtomicBool::new(true));
    (
        GetIsAlive {
            is_alive: is_alive.clone(),
        },
        SetIsAlive { is_alive },
    )
}

#[derive(Debug)]
struct GetProgress<T: Clone> {
    val: Arc<Mutex<T>>,
}

impl<T> GetProgress<T>
where
    T: Clone,
{
    pub fn get(&self) -> T {
        self.val.lock().unwrap().clone()
    }
}

#[derive(Debug)]
pub struct SetProgress<T: Clone> {
    val: Arc<Mutex<T>>,
}

impl<T> SetProgress<T>
where
    T: Clone,
{
    pub fn set(&self, new_val: T) {
        *self.val.lock().unwrap() = new_val;
    }
}

fn progress<T: Clone + Default>() -> (GetProgress<T>, SetProgress<T>) {
    let val = Arc::new(Mutex::new(T::default()));
    (GetProgress { val: val.clone() }, SetProgress { val })
}

pub fn run_in_background<P, T>(
    closure: impl FnOnce(GetIsAlive, SetProgress<P>) -> T + Send + 'static,
) -> BackgroundHandle<P, T>
where
    T: Send + 'static,
    P: Clone + Default + Send + 'static,
{
    let (get_alive, set_alive) = is_alive();
    let (get_prog, set_prog) = progress();
    BackgroundHandle {
        alive: set_alive,
        progress: get_prog,
        hndl: std::thread::spawn(move || closure(get_alive, set_prog)),
    }
}
