use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError, channel},
    time::Duration,
};

use iceoryx2::{
    config::Config,
    node::NodeBuilder,
    prelude::{AllocationStrategy, FileName, SemanticString, ZeroCopySend},
    service::ipc,
};
use serde::{Deserialize, Serialize};

use crate::AppResult;

const CYCLE_TIME: Duration = Duration::from_millis(100);

pub struct Channel<T> {
    send: Sender<T>,
    recv: Receiver<T>,
}

impl<T> Channel<T>
where
    T: Sync + Send + 'static + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new() -> Self {
        let (send, trecv) = channel::<T>();
        let (tsend, recv) = channel::<T>();

        std::thread::spawn(move || -> AppResult<()> {
            let node = NodeBuilder::new()
                .config(&Self::ipc_config())
                .create::<ipc::Service>()?;
            let service = node
                .service_builder(&"tabiew/broadcast/v1".try_into()?)
                .publish_subscribe::<[u8]>()
                .user_header::<Header>()
                .max_nodes(32)
                .max_publishers(32)
                .max_subscribers(32)
                .history_size(0)
                .open_or_create()?;

            let subscriber = service.subscriber_builder().create()?;
            let publisher = service
                .publisher_builder()
                .initial_max_slice_len(65_536)
                .allocation_strategy(AllocationStrategy::PowerOfTwo)
                .create()?;

            loop {
                loop {
                    match trecv.try_recv() {
                        Ok(msg) => {
                            let buf = postcard::to_allocvec(&msg)?;
                            let mut sample = publisher.loan_slice(buf.len())?;
                            sample.user_header_mut().pid = std::process::id();
                            sample.payload_mut().copy_from_slice(&buf);
                            sample.send()?;
                        }
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return Ok(()),
                    }
                }

                while let Some(sample) = subscriber.receive()? {
                    let msg = postcard::from_bytes(sample.payload())?;
                    if !sample.user_header().is_echo() {
                        tsend.send(msg)?;
                    }
                }

                std::thread::sleep(CYCLE_TIME)
            }
        });

        Self { send, recv }
    }

    pub fn recv(&self) -> Option<T> {
        self.recv.try_recv().ok()
    }

    pub fn recv_timeout(&self, duration: Duration) -> Option<T> {
        self.recv.recv_timeout(duration).ok()
    }

    pub fn send(&self, msg: T) {
        let _ = self.send.send(msg);
    }

    fn ipc_config() -> Config {
        let mut config = Config::default();
        config.global.prefix = FileName::new(b"tabiew_").unwrap();
        config
    }
}

impl<T> Default for Channel<T>
where
    T: Sync + Send + 'static + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, ZeroCopySend)]
#[repr(C)]
struct Header {
    pid: u32,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            pid: std::process::id(),
        }
    }
}

impl Header {
    pub fn is_echo(&self) -> bool {
        self.pid == std::process::id()
    }
}
