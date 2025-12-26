use std::{
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, TryRecvError, channel},
    },
    time::{Duration, Instant},
};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use itertools::Itertools;
use polars::{frame::DataFrame, prelude::IdxCa};

use rayon::prelude::*;

use crate::misc::polars_ext::AnyValueExt;

pub trait Score {
    fn score(&self, a: &str, b: &str) -> Option<i64>;
}

#[derive(Default)]
pub struct Skim {
    matcher: SkimMatcherV2,
}

impl Debug for Skim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Skim()")
    }
}

impl Score for Skim {
    fn score(&self, a: &str, b: &str) -> Option<i64> {
        self.matcher.fuzzy_match(a, b)
    }
}

#[derive(Debug, Default)]
pub struct Contain;

impl Score for Contain {
    fn score(&self, a: &str, b: &str) -> Option<i64> {
        a.contains(b).then_some(1)
    }
}

#[derive(Debug)]
pub struct Search<S> {
    pat: String,
    df: SyncDataFrame,
    _alive: SetFalseOnDrop,
    score: PhantomData<S>,
}

impl<S> Search<S>
where
    S: Score + Default + Sync + Send + 'static,
{
    pub fn new(df: DataFrame, pat: String) -> Self {
        let sync_df = SyncDataFrame::new();
        let alive = Arc::new(AtomicBool::new(true));
        if pat.is_empty() {
            // avoid search
            sync_df.insert(df);
            Self {
                df: sync_df,
                _alive: SetFalseOnDrop(alive),
                score: Default::default(),
                pat,
            }
        } else {
            // search
            // communication between search and collector threads
            let (tx, rx) = channel();

            // search thread
            std::thread::spawn({
                let matcher = S::default();
                let alive = alive.clone();
                let df = df.clone();
                let pat = pat.clone();
                move || {
                    let _ = df
                        .iter()
                        .flat_map(|series| series.iter().enumerate())
                        .par_bridge()
                        .take_any_while(|_| alive.load(Ordering::Relaxed))
                        .filter_map(|(idx, value)| {
                            let value = value.into_multi_line();
                            if value == pat {
                                Some((idx, i64::MAX))
                            } else {
                                // matcher.fuzzy_match(&value, &pat).map(|score| (idx, score))
                                matcher.score(&value, &pat).map(|score| (idx, score))
                            }
                        })
                        .try_for_each(|(idx, score)| tx.send((idx as u32, score)));
                }
            });

            // collector thread
            std::thread::spawn({
                let sync_df = sync_df.clone();
                move || {
                    let mut interval = Interval::new(Duration::from_millis(100));
                    let mut idx_score = HashMap::new();
                    let mut recv = ConnectionAware::new(rx);
                    while recv.connected() {
                        //do operations
                        for (idx, new_score) in recv.by_ref() {
                            idx_score
                                .entry(idx)
                                .and_modify(|score| *score = new_score.max(*score))
                                .or_insert(new_score);
                        }

                        sync_df.insert(
                            df.take(&IdxCa::new_vec(
                                "name".into(),
                                idx_score
                                    .iter()
                                    .sorted_by_key(|(idx, score)| (-*score, *idx))
                                    .map(|(idx, _)| *idx)
                                    .collect(),
                            ))
                            .unwrap(),
                        );
                        interval.sleep();
                    }
                }
            });
            Self {
                df: sync_df,
                _alive: SetFalseOnDrop(alive),
                pat,
                score: Default::default(),
            }
        }
    }

    pub fn latest(&self) -> Option<DataFrame> {
        self.df.take()
    }

    pub fn pattern(&self) -> &str {
        &self.pat
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

#[derive(Debug, Clone)]
struct SetFalseOnDrop(Arc<AtomicBool>);

impl Drop for SetFalseOnDrop {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Relaxed);
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
pub struct Interval {
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
