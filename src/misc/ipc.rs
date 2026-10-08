use std::{
    sync::{
        LazyLock, Mutex,
        mpsc::{Receiver, Sender, TryRecvError, channel},
    },
    time::Duration,
};

use iceoryx2::{
    config::Config,
    node::NodeBuilder,
    prelude::{AllocationStrategy, FileName, SemanticString, ZeroCopySend},
    service::ipc::{self},
};
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

use crate::{
    AppResult,
    misc::{sql::BackendSchema, unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown},
};

const CYCLE_TIME: Duration = Duration::from_millis(100);
static INTER_PROC: LazyLock<Mutex<InterProc>> = LazyLock::new(|| Mutex::new(InterProc::new()));

struct InterProc {
    send: Sender<Message>,
    recv: Receiver<Message>,
}

impl InterProc {
    pub fn new() -> Self {
        let (send, trecv) = channel::<Message>();
        let (tsend, recv) = channel::<Message>();

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
                            let buf = to_allocvec(&msg)?;
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
                    let msg = from_bytes(sample.payload())?;
                    if !sample.user_header().is_echo() {
                        tsend.send(msg)?;
                    }
                }

                std::thread::sleep(CYCLE_TIME)
            }
        });

        Self { send, recv }
    }

    fn recv(&self) -> Option<Message> {
        self.recv.try_recv().ok()
    }

    fn recv_timeout(&self, duration: Duration) -> Option<Message> {
        self.recv.recv_timeout(duration).ok()
    }

    fn send(&self, msg: Message) {
        let _ = self.send.send(msg);
    }

    fn ipc_config() -> Config {
        let mut config = Config::default();
        config.global.prefix = FileName::new(b"tabiew_").unwrap();
        config
    }
}

impl Default for InterProc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Ps,
    PsReply { pid: u32 },
    Schema { pid: u32 },
    SchemaReplay { pid: u32, schema: BackendSchema },
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

pub fn send(msg: Message) {
    INTER_PROC.lock().unwrap_or_graceful_shutdown().send(msg);
}

pub fn recv() -> Option<Message> {
    INTER_PROC.lock().unwrap_or_graceful_shutdown().recv()
}

pub fn recv_timeout(duration: Duration) -> Option<Message> {
    INTER_PROC
        .lock()
        .unwrap_or_graceful_shutdown()
        .recv_timeout(duration)
}

pub fn recv_iter() -> impl Iterator<Item = Message> {
    std::iter::from_fn(recv)
}

pub fn recv_iter_timeout(duration: Duration) -> impl Iterator<Item = Message> {
    std::iter::from_fn(move || recv_timeout(duration))
}

pub mod ops {
    use std::time::Duration;

    use itertools::Itertools;

    use crate::misc::sql::BackendSchema;

    use super::Message;

    pub fn fetch_other_process() -> Vec<u32> {
        super::send(super::Message::Ps);
        super::recv_iter_timeout(Duration::from_millis(2000))
            .filter_map(|msg| {
                if let super::Message::PsReply { pid } = msg {
                    Some(pid)
                } else {
                    None
                }
            })
            .collect_vec()
    }

    pub fn fetch_sql_backend(pid: u32) -> Option<BackendSchema> {
        super::send(super::Message::Schema { pid });
        super::recv_iter_timeout(Duration::from_millis(2000)).find_map(|msg| {
            if let Message::SchemaReplay { pid: tpid, schema } = msg
                && pid == tpid
            {
                Some(schema.clone())
            } else {
                None
            }
        })
    }
}
