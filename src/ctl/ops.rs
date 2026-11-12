use std::{
    io::{IsTerminal, Write},
    path::PathBuf,
    process::id,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::anyhow;
use itertools::Itertools;

use crate::{
    AppResult,
    ctl::types::{ExportSpec, ImportSpec, Rows, Schema, Shape, Tabs},
};

use super::Message;

const TIMEOUT: Duration = Duration::from_millis(2000);
const PING: Duration = Duration::from_secs(1);
const DEAD: Duration = Duration::from_secs(5);

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

fn job<T>(
    pid: u32,
    msg: Message,
    verb: &str,
    cancellable: bool,
    mut reply: impl FnMut(Message) -> Option<Result<T, String>>,
) -> AppResult<T> {
    let interrupted = interrupted();
    let tty = std::io::stderr().is_terminal();
    let clear = || {
        if tty {
            eprint!("\r\x1b[2K");
        }
    };
    msg.send();
    let started = Instant::now();
    let mut acked = false;
    let mut seen = Instant::now();
    let mut pinged = Instant::now();
    let mut cancelled: Option<Instant> = None;
    let mut frame = 0;
    loop {
        if cancelled.is_none() && interrupted.load(Ordering::Relaxed) {
            clear();
            if !cancellable {
                return Err(anyhow!("Stopped waiting, the job keeps running in {pid}"));
            }
            Message::cancel(pid).send();
            cancelled = Some(Instant::now());
        }
        if cancelled.is_some_and(|at| at.elapsed() > TIMEOUT) {
            return Err(anyhow!("Cancelled, no confirmation from {pid}"));
        }
        match Message::recv_timeout(Duration::from_millis(100)) {
            Some(Message::Ack { pid: tpid, to } | Message::Pong { pid: tpid, to })
                if tpid == pid && to == id() =>
            {
                acked = true;
                seen = Instant::now();
            }
            Some(Message::Progress {
                pid: tpid,
                to,
                fraction,
            }) if tpid == pid && to == id() => {
                seen = Instant::now();
                if tty && cancelled.is_none() {
                    eprint!("\r\x1b[2K{}", bar(verb, fraction, frame));
                    let _ = std::io::stderr().flush();
                    frame += 1;
                }
            }
            Some(msg) => {
                if let Some(result) = reply(msg) {
                    clear();
                    return result.map_err(|err| anyhow!(err));
                }
            }
            None => (),
        }
        if !acked && started.elapsed() > TIMEOUT {
            clear();
            return Err(anyhow!(
                "No reply received from {pid} within {} seconds",
                TIMEOUT.as_secs()
            ));
        }
        if acked && pinged.elapsed() >= PING {
            Message::ping(pid).send();
            pinged = Instant::now();
        }
        if acked && seen.elapsed() > DEAD {
            clear();
            return Err(anyhow!(
                "{pid} stopped responding for {} seconds",
                DEAD.as_secs()
            ));
        }
    }
}

fn bar(verb: &str, fraction: Option<f64>, frame: usize) -> String {
    const SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let Some(fraction) = fraction else {
        return format!("{verb} {}", SPINNER[frame % SPINNER.len()]);
    };
    let columns = crossterm::terminal::size().map_or(80, |(columns, _)| columns as usize);
    let width = columns.saturating_sub(verb.len() + 7).min(40);
    let filled = (fraction.clamp(0.0, 1.0) * width as f64).round() as usize;
    format!(
        "{verb} {}{} {:>3.0}%",
        "█".repeat(filled),
        "░".repeat(width - filled),
        fraction * 100.0
    )
}

fn interrupted() -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    let set = flag.clone();
    std::thread::spawn(move || {
        if let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            && runtime.block_on(tokio::signal::ctrl_c()).is_ok()
        {
            set.store(true, Ordering::Relaxed);
        }
    });
    flag
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
    job(
        pid,
        Message::sql(pid, query),
        "Running",
        true,
        |msg| match msg {
            Message::SqlReplay {
                pid: tpid,
                to,
                result,
            } if tpid == pid && to == id() => Some(result),
            _ => None,
        },
    )
}

pub fn send_import(pid: u32, spec: impl Into<ImportSpec>) -> AppResult<Vec<String>> {
    job(
        pid,
        Message::import(pid, spec),
        "Importing",
        false,
        |msg| match msg {
            Message::ImportReply {
                pid: tpid,
                to,
                result,
            } if tpid == pid && to == id() => Some(result),
            _ => None,
        },
    )
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
    job(
        pid,
        Message::filter(pid, filter),
        "Filtering",
        true,
        |msg| match msg {
            Message::FilterReply {
                pid: tpid,
                to,
                result,
            } if tpid == pid && to == id() => Some(result),
            _ => None,
        },
    )
}

pub fn send_select(pid: u32, columns: String) -> AppResult<Shape> {
    job(
        pid,
        Message::select(pid, columns),
        "Selecting",
        true,
        |msg| match msg {
            Message::SelectReply {
                pid: tpid,
                to,
                result,
            } if tpid == pid && to == id() => Some(result),
            _ => None,
        },
    )
}

pub fn send_order(pid: u32, order_by: String) -> AppResult<Shape> {
    job(
        pid,
        Message::order(pid, order_by),
        "Ordering",
        true,
        |msg| match msg {
            Message::OrderReply {
                pid: tpid,
                to,
                result,
            } if tpid == pid && to == id() => Some(result),
            _ => None,
        },
    )
}

pub fn send_export(pid: u32, spec: impl Into<ExportSpec>) -> AppResult<PathBuf> {
    job(
        pid,
        Message::export(pid, spec),
        "Exporting",
        false,
        |msg| match msg {
            Message::ExportReply {
                pid: tpid,
                to,
                result,
            } if tpid == pid && to == id() => Some(result),
            _ => None,
        },
    )
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
