use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, Sender, TryRecvError, channel},
    },
    time::{Duration, Instant},
};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use polars::{frame::DataFrame, prelude::IdxCa};

use rayon::prelude::*;

use crate::misc::{polars_ext::AnyValueExt, type_ext::UnwrapOrGracefulShutdown};

type RowIndex = u32;
type SimScore = i64;

#[derive(Debug)]
pub struct Searcher {
    pat: String,
    sync_df: SyncDataFrame,
    alive: Arc<AtomicBool>,
}

impl Searcher {
    pub fn exact(df: DataFrame, pat: String) -> Self {
        Self::new::<Exact>(df, pat)
    }

    pub fn fuzzy(df: DataFrame, pat: String) -> Self {
        Self::new::<Skim>(df, pat)
    }

    fn new<S>(df: DataFrame, pat: String) -> Self
    where
        S: Score + Default + Sync + Send + 'static,
    {
        let sync_df = SyncDataFrame::new();
        let alive = Arc::new(AtomicBool::new(true));
        if pat.is_empty() {
            sync_df.insert(df);
        } else {
            sync_df.insert(df.clear());
            let (tx, rx) = channel();
            spawn_search_thread::<S>(df.clone(), pat.to_owned(), alive.clone(), tx);
            spawn_collector_thread(df, rx, sync_df.clone());
        }
        Self {
            sync_df,
            alive,
            pat,
        }
    }

    pub fn latest(&self) -> Option<DataFrame> {
        self.sync_df.take()
    }

    pub fn pattern(&self) -> &str {
        &self.pat
    }
}

impl Drop for Searcher {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Relaxed);
    }
}

trait Score {
    fn score(&self, a: &str, b: &str) -> Option<i64>;
}

#[derive(Default)]
struct Skim {
    matcher: SkimMatcherV2,
}

impl Score for Skim {
    fn score(&self, a: &str, b: &str) -> Option<i64> {
        self.matcher.fuzzy_match(a, b)
    }
}

#[derive(Default)]
struct Exact;

impl Score for Exact {
    fn score(&self, a: &str, b: &str) -> Option<i64> {
        a.contains(b).then_some(1)
    }
}

#[derive(Debug, Clone)]
struct SyncDataFrame(Arc<Mutex<Option<DataFrame>>>);

impl SyncDataFrame {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    fn insert(&self, df: DataFrame) {
        if let Ok(mut mut_grd) = self.0.lock() {
            *mut_grd = Some(df);
        }
    }

    fn take(&self) -> Option<DataFrame> {
        self.0.lock().ok().and_then(|mut mut_grd| mut_grd.take())
    }
}

#[derive(Debug)]
struct ConnectionAware<T> {
    connected: bool,
    recv: Receiver<T>,
}

impl<T> ConnectionAware<T> {
    fn new(recv: Receiver<T>) -> Self {
        ConnectionAware {
            connected: true,
            recv,
        }
    }

    fn connected(&self) -> bool {
        self.connected
    }
}

impl<T> Iterator for ConnectionAware<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.recv.try_recv() {
            Ok(v) => Some(v),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                self.connected = false;
                None
            }
        }
    }
}

#[derive(Debug)]
struct Interval {
    tick_rate: Duration,
    last_tick: Instant,
}

impl Interval {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            tick_rate,
            last_tick: Instant::now(),
        }
    }

    pub fn sleep(&mut self) {
        std::thread::sleep(self.tick_rate.saturating_sub(self.last_tick.elapsed()));
        self.last_tick = Instant::now();
    }
}

#[derive(Debug, Default)]
struct Scores {
    map: HashMap<RowIndex, SimScore>,
    bts: BTreeSet<(Reverse<SimScore>, RowIndex)>,
}

impl Scores {
    fn insert(&mut self, row: RowIndex, score: SimScore) {
        self.map
            .entry(row)
            .and_modify(|old_score| {
                if *old_score < score {
                    self.bts.remove(&(Reverse(*old_score), row));
                    *old_score = score;
                    self.bts.insert((Reverse(score), row));
                }
            })
            .or_insert_with(|| {
                self.bts.insert((Reverse(score), row));
                score
            });
    }

    fn indices(&self) -> impl Iterator<Item = RowIndex> {
        self.bts.iter().map(|(_, idx)| *idx)
    }
}

fn spawn_search_thread<S>(
    df: DataFrame,
    pat: String,
    alive: Arc<AtomicBool>,
    tx: Sender<(RowIndex, SimScore)>,
) where
    S: Score + Default + Sync,
{
    std::thread::spawn(move || {
        let matcher = S::default();
        let _ = df
            .columns()
            .iter()
            .flat_map(|column| column.as_materialized_series().iter().enumerate())
            .par_bridge()
            .take_any_while(|_| alive.load(Ordering::Relaxed))
            .filter_map(|(idx, value)| {
                let value = value.to_multi_line();
                if value == pat {
                    Some((idx, i64::MAX))
                } else {
                    matcher.score(&value, &pat).map(|score| (idx, score))
                }
            })
            .try_for_each(|(idx, score)| tx.send((idx as u32, score)));
    });
}

fn spawn_collector_thread(
    df: DataFrame,
    rx: Receiver<(RowIndex, SimScore)>,
    sync_df: SyncDataFrame,
) {
    std::thread::spawn(move || {
        let mut interval = Interval::new(Duration::from_millis(100));
        let mut scores = Scores::default();
        let mut recv = ConnectionAware::new(rx);
        let mut updated = false;
        while recv.connected() {
            let mut should_update = false;
            for (idx, new_score) in recv.by_ref() {
                should_update = true;
                scores.insert(idx, new_score);
            }

            if should_update {
                sync_df.insert(
                    df.take(&IdxCa::new_vec("name".into(), scores.indices().collect()))
                        .unwrap_or_default(),
                );
                updated = true;
            }
            interval.sleep();
        }
        if !updated {
            sync_df.insert(
                df.take(&IdxCa::new_vec("name".into(), scores.indices().collect()))
                    .unwrap_or_graceful_shutdown(),
            );
        }
    });
}
