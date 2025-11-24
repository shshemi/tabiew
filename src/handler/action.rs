use std::sync::{
    LazyLock, Mutex,
    mpsc::{Receiver, Sender, channel},
};

#[derive(Debug)]
pub enum Action {
    
}

impl Action {
    pub fn submit(self) {
        SHARED_CHANNEL.0.send(self).unwrap();
    }
    pub fn next() -> Option<Action> {
        SHARED_CHANNEL.1.try_lock().ok()?.try_recv().ok()?.into()
    }
}

static SHARED_CHANNEL: LazyLock<(Sender<Action>, Mutex<Receiver<Action>>)> = LazyLock::new(|| {
    let (send, recv) = channel();
    (send, Mutex::new(recv))
});
