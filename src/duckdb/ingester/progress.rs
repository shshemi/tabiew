use duckdb::{Connection, ProgressHandle};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};

pub trait IngestProgress: Send + Sync {
    fn fraction(&self) -> Option<f64>;
}

pub struct StepProgress {
    handle: OnceLock<ProgressHandle>,
    step: AtomicUsize,
    steps: usize,
    highest: AtomicU64,
}

impl StepProgress {
    pub fn new(steps: usize) -> Arc<Self> {
        Arc::new(Self {
            handle: OnceLock::new(),
            step: AtomicUsize::new(0),
            steps,
            highest: AtomicU64::new(0),
        })
    }
    pub fn start(&self, conn: &Connection) {
        self.handle.get_or_init(|| conn.progress_handle());
    }
    pub fn next(&self) {
        self.step.fetch_add(1, Ordering::Relaxed);
    }
}

impl IngestProgress for StepProgress {
    fn fraction(&self) -> Option<f64> {
        let query = self
            .handle
            .get()
            .and_then(|handle| handle.query_progress().percentage())
            .unwrap_or(0.0)
            / 100.0;
        let step = self.step.load(Ordering::Relaxed) as f64;
        let fraction = ((step + query) / self.steps as f64).min(1.0);
        let highest = self
            .highest
            .fetch_max(fraction.to_bits(), Ordering::Relaxed);
        Some(fraction.max(f64::from_bits(highest)))
    }
}

#[derive(Default)]
pub struct CountProgress {
    done: AtomicU64,
    total: AtomicU64,
}

impl CountProgress {
    pub fn new() -> Arc<Self> {
        Arc::default()
    }
    pub fn total(&self, total: u64) {
        self.total.store(total, Ordering::Relaxed);
    }
    pub fn set(&self, done: u64) {
        self.done.store(done, Ordering::Relaxed);
    }
    pub fn add(&self, done: u64) {
        self.done.fetch_add(done, Ordering::Relaxed);
    }
}

impl IngestProgress for CountProgress {
    fn fraction(&self) -> Option<f64> {
        let total = self.total.load(Ordering::Relaxed);
        (total > 0).then(|| (self.done.load(Ordering::Relaxed) as f64 / total as f64).min(1.0))
    }
}

pub struct Parts {
    pub done: usize,
    pub parts: Vec<Arc<dyn IngestProgress>>,
}

impl IngestProgress for Parts {
    fn fraction(&self) -> Option<f64> {
        let running = self
            .parts
            .iter()
            .map(|part| part.fraction().unwrap_or(0.0))
            .sum::<f64>();
        Some((self.done as f64 + running) / (self.done + self.parts.len()) as f64)
    }
}
