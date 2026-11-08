use std::{
    sync::{
        OnceLock,
        mpsc::{Sender, channel},
    },
    thread::JoinHandle,
};

use crate::backend::db::Database;

pub fn submit(req: impl FnOnce(&mut Database) + Send + 'static) {
    static INSTANCE: OnceLock<Worker> = OnceLock::new();
    INSTANCE.get_or_init(|| Worker::default()).submit(req);
}

type BoxedAction = Box<dyn FnOnce(&mut Database) + Send + 'static>;

#[derive(Debug)]
struct Worker {
    send: Option<Sender<BoxedAction>>,
    hndl: Option<JoinHandle<()>>,
}

impl Worker {
    fn submit(&self, req: impl FnOnce(&mut Database) + Send + 'static) {
        if let Some(send) = self.send.as_ref() {
            send.send(Box::new(req)).unwrap();
        }
    }
}

impl Default for Worker {
    fn default() -> Self {
        let mut db = Database::new().unwrap();
        let (send, recv) = channel::<BoxedAction>();
        let hndl = Some(std::thread::spawn(move || {
            loop {
                let Ok(req) = recv.recv() else {
                    break;
                };
                req(&mut db);
            }
        }));
        Self {
            send: Some(send),
            hndl,
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.send.take();
        if let Some(hndl) = self.hndl.take() {
            let _ = hndl.join();
        }
    }
}
