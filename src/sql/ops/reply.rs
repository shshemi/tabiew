use std::sync::mpsc::TryRecvError;

use anyhow::anyhow;

use crate::AppResult;

#[derive(Debug)]
pub struct Sender<T> {
    send: std::sync::mpsc::Sender<AppResult<T>>,
}

impl<T> Sender<T> {
    pub fn send(&self, t: AppResult<T>) {
        let _ = self.send.send(t);
    }
}

#[derive(Debug)]
pub struct Receiver<T> {
    recv: std::sync::mpsc::Receiver<AppResult<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Next<T> {
        match self.recv.try_recv() {
            Ok(v) => Next::Value(v),
            Err(TryRecvError::Empty) => Next::Empty,
            Err(TryRecvError::Disconnected) => Next::Done,
        }
    }

    pub fn recv_blocking(&self) -> AppResult<T> {
        self.recv
            .recv()
            .map_err(|_| anyhow!("sender dropped without sending a value"))?
    }
}

#[derive(Debug)]
pub enum Next<T> {
    Value(AppResult<T>),
    Empty,
    Done,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (send, recv) = std::sync::mpsc::channel::<AppResult<T>>();
    (Sender { send }, Receiver { recv })
}
