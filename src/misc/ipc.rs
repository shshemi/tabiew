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
    prelude::{FileName, SemanticString, ZeroCopySend},
    service::ipc::{self},
};
use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

use crate::{AppResult, misc::unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown};

const CYCLE_TIME: Duration = Duration::from_millis(100);
static INTER_PROC: LazyLock<Mutex<InterProc>> = LazyLock::new(|| Mutex::new(InterProc::new()));

struct InterProc {
    send: Sender<Message>,
    recv: Receiver<Message>,
}

impl InterProc {
    pub fn new() -> Self {
        let (send, trecv) = channel();
        let (tsend, recv) = channel();

        std::thread::spawn(move || -> AppResult<()> {
            let node = NodeBuilder::new()
                .config(&Self::ipc_config())
                .create::<ipc::Service>()?;
            let service = node
                .service_builder(&"tabiew/broadcast/v1".try_into()?)
                .publish_subscribe::<Envelope>()
                .max_nodes(32)
                .max_publishers(32)
                .max_subscribers(32)
                .subscriber_max_buffer_size(8)
                .history_size(0)
                .open_or_create()?;

            let subscriber = service.subscriber_builder().create()?;
            let publisher = service.publisher_builder().create()?;

            loop {
                loop {
                    match trecv.try_recv() {
                        Ok(msg) => {
                            if let Ok(packet) = Envelope::from_message(msg) {
                                publisher.send_copy(packet)?;
                            }
                        }
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return Ok(()),
                    }
                }

                while let Some(envelope) = subscriber.receive().ok().flatten() {
                    if !envelope.is_echo()
                        && let Some(msg) = envelope.to_message()
                    {
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
}

impl Message {
    pub fn send(self) {
        send(self);
    }
}

const PACKET_SIZE: usize = 1024;
#[derive(Debug, ZeroCopySend)]
#[repr(C)]
struct Envelope {
    pid: u32,
    buf: [u8; PACKET_SIZE],
}

impl Envelope {
    fn from_message(msg: Message) -> AppResult<Envelope> {
        let mut buf = [0; PACKET_SIZE];
        to_slice(&msg, &mut buf)?;
        Ok(Envelope {
            pid: std::process::id(),
            buf,
        })
    }

    pub fn is_echo(&self) -> bool {
        self.pid == std::process::id()
    }

    pub fn to_message(&self) -> Option<Message> {
        from_bytes(&self.buf).ok()
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
}
