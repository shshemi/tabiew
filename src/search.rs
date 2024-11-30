use std::{
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use polars::{frame::DataFrame, prelude::BooleanChunked};

use rayon::prelude::*;

use crate::utils::polars_ext::FuzzyCmp;

#[derive(Debug)]
pub struct Search {
    latest: Arc<Mutex<DataFrame>>,
    send: Sender<String>,
    hndl: JoinHandle<DataFrame>,
}

impl Search {
    pub fn new(df: DataFrame) -> Self {
        let latest = Arc::new(Mutex::new(df.clone()));
        let (send, recv) = channel();
        let hndl = std::thread::spawn({
            let latest = latest.clone();
            move || {
                let mut recv: PeekableReceiver<String> = PeekableReceiver::new(recv);
                while let Ok(pat) = recv.recv() {
                    let mut mask = vec![false; df.height()];

                    for idx in Task::new(df.clone(), pat).iter() {
                        mask[idx] = true;

                        *latest.lock().unwrap() = df
                            .filter(&BooleanChunked::from_iter(mask.iter().copied()))
                            .unwrap();

                        if recv.check() {
                            break;
                        }
                    }
                    *latest.lock().unwrap() = df.filter(&BooleanChunked::from_iter(mask)).unwrap();
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

    pub fn latest(&self) -> DataFrame {
        self.latest.lock().unwrap().clone()
    }
}

struct PeekableReceiver<T> {
    latest: Option<T>,
    recv: Receiver<T>,
}

impl<T> PeekableReceiver<T> {
    fn new(recv: Receiver<T>) -> Self {
        Self { latest: None, recv }
    }

    fn check(&mut self) -> bool {
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
                .filter_map(|(idx, value)| (value.fuzzy_cmp(&pat)).then_some(idx))
                .try_for_each(|idx| send.send(idx));
            df
        });
        Self { recv, _hndl: hndl }
    }

    fn iter(&self) -> std::sync::mpsc::Iter<usize> {
        self.recv.iter()
    }
}
