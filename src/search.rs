use std::{
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use polars::{frame::DataFrame, prelude::BooleanChunked};

use rayon::prelude::*;

use crate::utils::polars_ext::IntoString;

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
pub struct Search {
    latest: SyncDataFrame,
    send: Sender<String>,
    hndl: JoinHandle<DataFrame>,
}

impl Search {
    pub fn new(df: DataFrame) -> Self {
        let latest = SyncDataFrame::new();
        let (send, recv) = channel();
        let hndl = std::thread::spawn({
            let latest = latest.clone();
            move || {
                let mut recv: ConnectionAwareReceiver<String> = ConnectionAwareReceiver::new(recv);
                while let Ok(pat) = recv.recv() {
                    let mut mask = vec![false; df.height()];

                    for idx in IndexSearch::new(df.clone(), pat).iter() {
                        mask[idx] = true;

                        latest.insert(
                            df.filter(&BooleanChunked::from_iter(mask.iter().copied()))
                                .unwrap(),
                        );

                        if recv.disconnected() {
                            break;
                        }
                    }
                    latest.insert(
                        df.filter(&BooleanChunked::from_iter(mask.iter().copied()))
                            .unwrap(),
                    );
                }
                df
            }
        });
        Self { latest, send, hndl }
    }

    pub fn search(&self, pat: String) {
        let _ = self.send.send(pat);
    }

    pub fn into_original_data_frame(self) -> DataFrame {
        drop(self.send);
        self.hndl.join().unwrap()
    }

    pub fn latest(&self) -> Option<DataFrame> {
        self.latest.take()
    }
}

struct ConnectionAwareReceiver<T> {
    latest: Option<T>,
    recv: Receiver<T>,
}

impl<T> ConnectionAwareReceiver<T> {
    fn new(recv: Receiver<T>) -> Self {
        Self { latest: None, recv }
    }

    fn disconnected(&mut self) -> bool {
        match self.recv.try_recv() {
            Ok(val) => {
                self.latest = val.into();
                true
            }
            Err(TryRecvError::Disconnected) => true,
            Err(TryRecvError::Empty) => false,
        }
    }

    fn recv(&mut self) -> Result<T, std::sync::mpsc::RecvError> {
        self.latest
            .take()
            .map(|val| Ok(val))
            .unwrap_or(self.recv.recv())
    }
}

#[derive(Debug)]
struct IndexSearch {
    recv: Receiver<usize>,
    _hndl: JoinHandle<DataFrame>,
}

impl IndexSearch {
    fn new(df: DataFrame, pat: String) -> Self {
        let matcher = SkimMatcherV2::default();
        let (send, recv) = channel::<usize>();
        let hndl = std::thread::spawn(move || {
            let _ = df
                .iter()
                .flat_map(|series| series.iter().enumerate())
                .par_bridge()
                .filter_map(|(idx, value)| {
                    matcher.fuzzy_match(&value.into_string(), &pat).map(|_| idx)
                })
                .try_for_each(|idx| send.send(idx));
            df
        });
        Self { recv, _hndl: hndl }
    }

    fn iter(&self) -> std::sync::mpsc::Iter<usize> {
        self.recv.iter()
    }
}
