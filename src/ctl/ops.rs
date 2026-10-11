use std::{path::PathBuf, time::Duration};

use anyhow::anyhow;
use itertools::Itertools;

use crate::{
    AppResult,
    ctl::types::{ExportSpec, ImportSpec, Rows, Schema, Shape, Tabs},
};

use super::Message;

const TIMEOUT: Duration = Duration::from_millis(2000);

fn request<T>(pid: u32, msg: Message, reply: impl FnMut(Message) -> Option<T>) -> AppResult<T> {
    msg.send();
    Message::recv_iter_timeout(TIMEOUT)
        .find_map(reply)
        .ok_or_else(|| {
            anyhow!(
                "No reply received from {pid} within {} seconds",
                TIMEOUT.as_secs()
            )
        })
}

pub fn fetch_other_process() -> AppResult<Vec<u32>> {
    Message::ps().send();
    Ok(Message::recv_iter_timeout(TIMEOUT)
        .filter_map(|msg| {
            if let Message::PsReply { pid } = msg {
                Some(pid)
            } else {
                None
            }
        })
        .collect_vec())
}

pub fn fetch_sql_backend(pid: u32) -> AppResult<Schema> {
    request(pid, Message::schema(pid), |msg| match msg {
        Message::SchemaReplay { pid: tpid, schema } if tpid == pid => Some(schema),
        _ => None,
    })
}

pub fn send_sql_query(pid: u32, query: String) -> AppResult<Shape> {
    request(pid, Message::sql(pid, query), |msg| match msg {
        Message::SqlReplay { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn send_import(pid: u32, spec: impl Into<ImportSpec>) -> AppResult<Vec<String>> {
    request(pid, Message::import(pid, spec), |msg| match msg {
        Message::ImportReply { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn fetch_tabs(pid: u32) -> AppResult<Tabs> {
    request(pid, Message::tabs(pid), |msg| match msg {
        Message::TabsReply { pid: tpid, tabs } if tpid == pid => Some(tabs),
        _ => None,
    })
}

pub fn switch_tab(pid: u32, tab_idx: usize) -> AppResult<usize> {
    request(pid, Message::tab_switch(pid, tab_idx), |msg| match msg {
        Message::TabSwitchReply {
            pid: tpid,
            new_tab_idx,
        } if tpid == pid => Some(new_tab_idx),
        _ => None,
    })?
    .ok_or_else(|| anyhow!("Tab {tab_idx} does not exist in {pid}"))
}

pub fn send_filter(pid: u32, filter: String) -> AppResult<Shape> {
    request(pid, Message::filter(pid, filter), |msg| match msg {
        Message::FilterReply { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn send_select(pid: u32, columns: String) -> AppResult<Shape> {
    request(pid, Message::select(pid, columns), |msg| match msg {
        Message::SelectReply { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn send_order(pid: u32, order_by: String) -> AppResult<Shape> {
    request(pid, Message::order(pid, order_by), |msg| match msg {
        Message::OrderReply { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn send_export(pid: u32, spec: impl Into<ExportSpec>) -> AppResult<PathBuf> {
    request(pid, Message::export(pid, spec), |msg| match msg {
        Message::ExportReply { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn select_row(pid: u32, row_idx: usize) -> AppResult<usize> {
    request(pid, Message::row_select(pid, row_idx), |msg| match msg {
        Message::RowSelectReply {
            pid: tpid,
            new_row_idx,
        } if tpid == pid => Some(new_row_idx),
        _ => None,
    })?
    .ok_or_else(|| anyhow!("Row {row_idx} does not exist in {pid}"))
}

pub fn fetch_rows(pid: u32, offset: Option<usize>, count: Option<usize>) -> AppResult<Rows> {
    request(pid, Message::rows(pid, offset, count), |msg| match msg {
        Message::RowsReply { pid: tpid, result } if tpid == pid => Some(result),
        _ => None,
    })?
    .map_err(|err| anyhow!(err))
}

pub fn remove_tab(pid: u32, tab_idx: usize) -> AppResult<usize> {
    request(pid, Message::rm_tab(pid, tab_idx), |msg| match msg {
        Message::RmTabReply {
            pid: tpid,
            removed_tab_idx,
        } if tpid == pid => Some(removed_tab_idx),
        _ => None,
    })?
    .ok_or_else(|| anyhow!("Tab {tab_idx} does not exist in {pid}"))
}
