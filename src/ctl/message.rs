use std::{
    process::id,
    sync::{LazyLock, Mutex},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{
    ctl::types::{ImportSpec, Schema},
    misc::{ipc, sql::BackendSchema, unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown},
};

static IPC: LazyLock<Mutex<ipc::Channel<Message>>> =
    LazyLock::new(|| Mutex::new(Default::default()));

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Ps,
    PsReply { pid: u32 },
    Schema { pid: u32 },
    SchemaReplay { pid: u32, schema: Schema },
    Sql { pid: u32, query: String },
    SqlReplay { pid: u32, msg: String },
    Import { pid: u32, spec: ImportSpec },
    ImportReply { pid: u32, msg: String },
}

impl Message {
    pub fn ps() -> Self {
        Message::Ps
    }

    pub fn ps_reply() -> Self {
        Message::PsReply { pid: id() }
    }

    pub fn schema(pid: u32) -> Self {
        Message::Schema { pid }
    }

    pub fn schema_replay(schema: &BackendSchema) -> Self {
        Message::SchemaReplay {
            pid: id(),
            schema: schema.into(),
        }
    }

    pub fn sql(pid: u32, query: impl Into<String>) -> Self {
        Message::Sql {
            pid,
            query: query.into(),
        }
    }

    pub fn sql_replay(msg: impl Into<String>) -> Self {
        Message::SqlReplay {
            pid: id(),
            msg: msg.into(),
        }
    }

    pub fn import(pid: u32, spec: impl Into<ImportSpec>) -> Self {
        Message::Import {
            pid,
            spec: spec.into(),
        }
    }

    pub fn import_reply(msg: impl Into<String>) -> Self {
        Message::ImportReply {
            pid: id(),
            msg: msg.into(),
        }
    }

    pub fn send(self) {
        IPC.lock().unwrap_or_graceful_shutdown().send(self);
    }

    pub fn recv() -> Option<Message> {
        IPC.lock().unwrap_or_graceful_shutdown().recv()
    }

    pub fn recv_timeout(duration: Duration) -> Option<Message> {
        IPC.lock()
            .unwrap_or_graceful_shutdown()
            .recv_timeout(duration)
    }

    pub fn recv_iter() -> impl Iterator<Item = Message> {
        std::iter::from_fn(Self::recv)
    }

    pub fn recv_iter_timeout(duration: Duration) -> impl Iterator<Item = Message> {
        std::iter::from_fn(move || Self::recv_timeout(duration))
    }
}
