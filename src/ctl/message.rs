use std::{
    path::PathBuf,
    process::id,
    sync::{LazyLock, Mutex},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{
    ctl::types::{ExportSpec, ImportSpec, Schema, Shape, Tabs},
    misc::{ipc, sql::BackendSchema, unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown},
    tui,
};

static IPC: LazyLock<Mutex<ipc::Channel<Message>>> =
    LazyLock::new(|| Mutex::new(Default::default()));

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Ps,
    PsReply {
        pid: u32,
    },
    Schema {
        pid: u32,
    },
    SchemaReplay {
        pid: u32,
        schema: Schema,
    },
    Sql {
        pid: u32,
        query: String,
    },
    SqlReplay {
        pid: u32,
        result: Result<Shape, String>,
    },
    Import {
        pid: u32,
        spec: ImportSpec,
    },
    ImportReply {
        pid: u32,
        result: Result<Vec<String>, String>,
    },
    Tabs {
        pid: u32,
    },
    TabsReply {
        pid: u32,
        tabs: Tabs,
    },
    TabSwitch {
        pid: u32,
        tab_idx: usize,
    },
    TabSwitchReply {
        pid: u32,
        new_tab_idx: Option<usize>,
    },
    RmTab {
        pid: u32,
        tab_idx: usize,
    },
    RmTabReply {
        pid: u32,
        removed_tab_idx: Option<usize>,
    },
    RowSelect {
        pid: u32,
        row_idx: usize,
    },
    RowSelectReply {
        pid: u32,
        new_row_idx: Option<usize>,
    },
    Filter {
        pid: u32,
        filter: String,
    },
    FilterReply {
        pid: u32,
        result: Result<Shape, String>,
    },
    Select {
        pid: u32,
        columns: String,
    },
    SelectReply {
        pid: u32,
        result: Result<Shape, String>,
    },
    Order {
        pid: u32,
        order_by: String,
    },
    OrderReply {
        pid: u32,
        result: Result<Shape, String>,
    },
    Export {
        pid: u32,
        spec: ExportSpec,
    },
    ExportReply {
        pid: u32,
        result: Result<PathBuf, String>,
    },
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

    pub fn sql_replay(result: Result<Shape, String>) -> Self {
        Message::SqlReplay { pid: id(), result }
    }

    pub fn import(pid: u32, spec: impl Into<ImportSpec>) -> Self {
        Message::Import {
            pid,
            spec: spec.into(),
        }
    }

    pub fn import_reply(result: Result<Vec<String>, String>) -> Self {
        Message::ImportReply { pid: id(), result }
    }

    pub fn tabs(pid: u32) -> Self {
        Message::Tabs { pid }
    }

    pub fn tabs_reply(tabs: &tui::tabs::Tabs) -> Self {
        Message::TabsReply {
            pid: id(),
            tabs: tabs.into(),
        }
    }

    pub fn tab_switch(pid: u32, tab_idx: usize) -> Self {
        Message::TabSwitch { pid, tab_idx }
    }

    pub fn tab_switch_reply(new_tab_idx: Option<usize>) -> Self {
        Message::TabSwitchReply {
            pid: id(),
            new_tab_idx,
        }
    }

    pub fn filter(pid: u32, filter: impl Into<String>) -> Self {
        Message::Filter {
            pid,
            filter: filter.into(),
        }
    }

    pub fn filter_reply(result: Result<Shape, String>) -> Self {
        Message::FilterReply { pid: id(), result }
    }

    pub fn select(pid: u32, columns: impl Into<String>) -> Self {
        Message::Select {
            pid,
            columns: columns.into(),
        }
    }

    pub fn select_reply(result: Result<Shape, String>) -> Self {
        Message::SelectReply { pid: id(), result }
    }

    pub fn order(pid: u32, order_by: impl Into<String>) -> Self {
        Message::Order {
            pid,
            order_by: order_by.into(),
        }
    }

    pub fn order_reply(result: Result<Shape, String>) -> Self {
        Message::OrderReply { pid: id(), result }
    }

    pub fn export(pid: u32, spec: impl Into<ExportSpec>) -> Self {
        Message::Export {
            pid,
            spec: spec.into(),
        }
    }

    pub fn export_reply(result: Result<PathBuf, String>) -> Self {
        Message::ExportReply { pid: id(), result }
    }

    pub fn row_select(pid: u32, row_idx: usize) -> Self {
        Message::RowSelect { pid, row_idx }
    }

    pub fn row_select_reply(new_row_idx: Option<usize>) -> Self {
        Message::RowSelectReply {
            pid: id(),
            new_row_idx,
        }
    }

    pub fn rm_tab(pid: u32, tab_idx: usize) -> Self {
        Message::RmTab { pid, tab_idx }
    }

    pub fn rm_tab_reply(removed_tab_idx: Option<usize>) -> Self {
        Message::RmTabReply {
            pid: id(),
            removed_tab_idx,
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
