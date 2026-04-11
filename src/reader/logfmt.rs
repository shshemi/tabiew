use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::{Duration, Instant};

use indexmap::IndexMap;
use logfmt_zerocopy::Logfmt;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, Column, Schema},
};

use crate::{
    AppResult,
    args::Args,
    misc::stdin::{stdin, stdin_raw_locked},
    reader::{
        NamedFrames, ReadToDataFrames, Source, StreamEvent, StreamToDataFrames,
        traits::StreamingConfig,
    },
};

#[derive(Debug, Default)]
pub struct LogfmtToDataFrame {}

impl LogfmtToDataFrame {
    pub fn from_args(_args: &Args) -> Self {
        LogfmtToDataFrame::default()
    }
}

impl ReadToDataFrames for LogfmtToDataFrame {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let contents = match &input {
            Source::File(path_buf) => fs::read_to_string(path_buf)?,
            Source::Stdin => {
                let mut s = String::new();
                stdin().read_to_string(&mut s)?;
                s
            }
        };
        let df = parse_logfmt_lines(contents.lines())?;
        Ok([(input.table_name(), df)].into())
    }
}

/// Parse a collection of logfmt lines into a DataFrame. Extracted from the
/// eager reader so the streaming reader can reuse it per batch.
pub(crate) fn parse_logfmt_lines<'a, I>(lines: I) -> AppResult<DataFrame>
where
    I: IntoIterator<Item = &'a str> + Clone,
{
    // We need the total row count up front because a column that first
    // appears late still needs null padding for earlier rows in the batch.
    let row_count = lines.clone().into_iter().count();
    let mut data: IndexMap<&str, Vec<AnyValue<'_>>> = IndexMap::new();

    for (row, line) in lines.into_iter().enumerate() {
        for (col, value) in line.logfmt() {
            insert(&mut data, col, row, row_count, value);
        }
    }

    let df = DataFrame::new_infer_height(
        data.into_iter()
            .map(|(name, values)| Column::new(name.into(), values))
            .collect(),
    )?;
    Ok(df)
}

fn insert<'k, 'v>(
    data: &mut IndexMap<&'k str, Vec<AnyValue<'v>>>,
    col: &'k str,
    row: usize,
    row_count: usize,
    value: &'v str,
) {
    if let Some(col) = data.get_mut(col) {
        col[row] = AnyValue::String(value);
    } else {
        let mut new_col = vec![AnyValue::Null; row_count];
        new_col[row] = AnyValue::String(value);
        data.insert(col, new_col);
    }
}

/// Streaming logfmt reader. Accumulates lines up to `config.batch_rows` or
/// `config.batch_ms`, parses each batch via `parse_logfmt_lines`, locks the
/// schema on the first batch, and coerces subsequent batches to that schema
/// (missing columns are null-filled; unknown new columns are dropped so the
/// upsert index stays stable).
pub struct LogfmtStreamReader {
    config: StreamingConfig,
}

impl LogfmtStreamReader {
    pub fn from_args(args: &Args) -> Self {
        Self {
            config: StreamingConfig::from_args(args),
        }
    }
}

impl StreamToDataFrames for LogfmtStreamReader {
    fn stream_to_data_frames(self: Box<Self>, input: Source, sender: SyncSender<StreamEvent>) {
        let name = input.table_name();
        let config = self.config;
        thread::spawn(move || {
            if !matches!(input, Source::Stdin) {
                let _ = sender.send(StreamEvent::Error {
                    name: name.clone(),
                    error: anyhow::anyhow!("streaming logfmt only supports stdin"),
                });
                return;
            }
            let reader = BufReader::new(stdin_raw_locked());
            run_logfmt_stream(reader, &name, config, &sender);
        });
    }
}

pub(crate) fn run_logfmt_stream<R: BufRead>(
    reader: R,
    name: &str,
    config: StreamingConfig,
    sender: &SyncSender<StreamEvent>,
) {
    let mut pending: Vec<String> = Vec::new();
    let mut locked: Option<Arc<Schema>> = None;
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_millis(config.batch_ms);
    let mut lines = reader.lines();

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if !line.is_empty() {
                    pending.push(line);
                }
            }
            Some(Err(err)) => {
                let _ = sender.send(StreamEvent::Error {
                    name: name.to_string(),
                    error: anyhow::anyhow!(err),
                });
                return;
            }
            None => {
                if !pending.is_empty()
                    && !flush_logfmt_batch(name, &mut pending, &mut locked, sender)
                {
                    return;
                }
                let _ = sender.send(StreamEvent::Eof {
                    name: name.to_string(),
                });
                return;
            }
        }

        let time_ready = last_flush.elapsed() >= flush_interval;
        let size_ready = pending.len() >= config.batch_rows;
        if !pending.is_empty()
            && (time_ready || size_ready)
            && !flush_logfmt_batch(name, &mut pending, &mut locked, sender)
        {
            return;
        }
        if time_ready || size_ready {
            last_flush = Instant::now();
        }
    }
}

fn flush_logfmt_batch(
    name: &str,
    pending: &mut Vec<String>,
    locked: &mut Option<Arc<Schema>>,
    sender: &SyncSender<StreamEvent>,
) -> bool {
    let lines: Vec<String> = std::mem::take(pending);
    let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let df = match parse_logfmt_lines(line_refs.iter().copied()) {
        Ok(df) => df,
        Err(err) => {
            let _ = sender.send(StreamEvent::Error {
                name: name.to_string(),
                error: anyhow::anyhow!(err),
            });
            return false;
        }
    };

    let df = match locked {
        Some(schema) => match conform_to_schema(df, schema) {
            Ok(d) => d,
            Err(err) => {
                let _ = sender.send(StreamEvent::Error {
                    name: name.to_string(),
                    error: anyhow::anyhow!(err),
                });
                return false;
            }
        },
        None => {
            let schema: Arc<Schema> = Arc::new(df.schema().as_ref().clone());
            *locked = Some(schema.clone());
            if sender
                .send(StreamEvent::Schema {
                    name: name.to_string(),
                    schema,
                })
                .is_err()
            {
                return false;
            }
            df
        }
    };

    sender
        .send(StreamEvent::Batch {
            name: name.to_string(),
            rows: df,
        })
        .is_ok()
}

/// Coerce a freshly parsed batch to match the locked schema: drop any new
/// columns and fill missing known columns with null strings.
fn conform_to_schema(mut df: DataFrame, schema: &Schema) -> AppResult<DataFrame> {
    let height = df.height();
    // Drop unknown columns.
    let unknown: Vec<String> = df
        .get_column_names()
        .into_iter()
        .filter(|n| !schema.contains(n))
        .map(|s| s.to_string())
        .collect();
    for name in unknown {
        let _ = df.drop_in_place(&name)?;
    }
    // Add missing columns as all-null, typed to match the locked schema so
    // a later vstack doesn't trip the "expected String, got null" guard.
    for (name, dtype) in schema.iter() {
        if df.column(name).is_err() {
            df.with_column(Column::full_null(name.clone(), height, dtype))?;
        }
    }
    // Reorder columns to the schema order.
    let ordered: Vec<String> = schema.iter_names().map(|s| s.to_string()).collect();
    let ordered_refs: Vec<&str> = ordered.iter().map(String::as_str).collect();
    let df = df.select(&ordered_refs)?;
    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::sync::mpsc::sync_channel;

    fn drain(rx: std::sync::mpsc::Receiver<StreamEvent>) -> Vec<StreamEvent> {
        let mut out = Vec::new();
        while let Ok(evt) = rx.recv() {
            out.push(evt);
        }
        out
    }

    #[test]
    fn locks_schema_and_drops_new_columns_in_later_batches() {
        // Batch 1: a, b. Batch 2: a, b, c — the `c` should be dropped.
        let data = b"a=1 b=2\na=3 b=4\na=5 b=6 c=7\na=8 b=9 c=10\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_logfmt_stream(
            io::Cursor::new(data),
            "t",
            StreamingConfig {
                batch_rows: 2,
                batch_ms: 10_000,
            },
            &tx,
        );
        drop(tx);
        let events = drain(rx);
        let batches: Vec<&DataFrame> = events
            .iter()
            .filter_map(|e| match e {
                StreamEvent::Batch { rows, .. } => Some(rows),
                _ => None,
            })
            .collect();
        assert_eq!(batches.len(), 2);
        for b in &batches {
            assert_eq!(b.get_column_names(), &["a", "b"]);
        }
    }

    #[test]
    fn missing_known_columns_are_null_filled() {
        // Batch 1 has a and b; batch 2 has only a. The "b" column must be
        // null-padded for rows in batch 2 so the schema stays stable.
        let data = b"a=1 b=2\na=3 b=4\na=5\na=6\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_logfmt_stream(
            io::Cursor::new(data),
            "t",
            StreamingConfig {
                batch_rows: 2,
                batch_ms: 10_000,
            },
            &tx,
        );
        drop(tx);
        let events = drain(rx);
        let batches: Vec<&DataFrame> = events
            .iter()
            .filter_map(|e| match e {
                StreamEvent::Batch { rows, .. } => Some(rows),
                _ => None,
            })
            .collect();
        assert_eq!(batches.len(), 2);
        let b2 = &batches[1];
        assert_eq!(b2.get_column_names(), &["a", "b"]);
        let b_col = b2.column("b").unwrap().str().unwrap();
        assert_eq!(b_col.get(0), None);
        assert_eq!(b_col.get(1), None);
    }
}
