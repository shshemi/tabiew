use std::{
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use polars::{frame::DataFrame, prelude::BooleanChunked};

use rayon::prelude::*;

use crate::utils::type_ext::HasSubsequence;


#[derive(Debug, Clone, Copy)]
enum State {
    Waiting,
    Searching,
}

#[derive(Debug)]
pub struct Search {
    latest: Arc<Mutex<DataFrame>>,
    send_stop: Sender<()>,
    send_pat: Sender<String>,
    hndl: JoinHandle<DataFrame>,
    state: Arc<Mutex<State>>,
}

impl Search {
    pub fn new(df: DataFrame) -> Self {
        let latest = Arc::new(Mutex::new(df.clone()));
        let (send_stop, recv_stop) = channel();
        let (send_pat, recv_pat) = channel();
        let state = Arc::new(Mutex::new(State::Waiting));
        let hndl = std::thread::spawn({
            let latest = latest.clone();
            let state = state.clone();
            move || {
                while let Ok(pat) = recv_pat.recv() {
                    *state.lock().unwrap() = State::Searching;
                    let mut mask = vec![false; df.height()];
                    for idx in Task::new(df.clone(), pat).iter() {
                        mask[idx] = true;
                        *latest.lock().unwrap() = df
                            .filter(&BooleanChunked::from_iter(mask.iter().copied()))
                            .unwrap();
                        match recv_stop.try_recv() {
                            Ok(()) | Err(TryRecvError::Disconnected) => break,
                            Err(TryRecvError::Empty) => (),
                        }
                    }
                    *latest.lock().unwrap() = df
                        .filter(&BooleanChunked::from_iter(mask.iter().copied()))
                        .unwrap();
                    *state.lock().unwrap() = State::Waiting;
                }
                df
            }
        });
        Self {
            latest,
            send_stop,
            send_pat,
            hndl,
            state,
        }
    }

    pub fn search(&self, pat: String) {
        let _ = self.send_pat.send(pat);
        if matches!(*self.state.lock().unwrap(), State::Searching) {
            let _ = self.send_stop.send(());
        }
    }

    pub fn into_original_data_frame(self) -> DataFrame {
        drop(self.send_pat);
        drop(self.send_stop);
        self.hndl.join().unwrap()
    }

    pub fn latest(&self) -> DataFrame {
        self.latest.lock().unwrap().clone()
    }
}

#[derive(Debug)]
struct Task {
    recv: Receiver<usize>,
    _hndl: JoinHandle<DataFrame>,
}

impl Task {
    fn new(df: DataFrame, pat: String) -> Self {
        let (send, recv) = channel::<usize>();
        let hndl = std::thread::spawn(move || {
            let _ = df
                .iter()
                .flat_map(|series| series.iter().enumerate())
                .par_bridge()
                .map(|(idx, value)| (idx, value.to_string()))
                .filter_map(|(idx, value)| (value.has_subsequence(&pat)).then_some(idx))
                .try_for_each(|idx| send.send(idx));
            df
        });
        Self { recv, _hndl: hndl }
    }

    fn iter(&self) -> std::sync::mpsc::Iter<usize> {
        self.recv.iter()
    }
}
