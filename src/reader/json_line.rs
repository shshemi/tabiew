use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::{Duration, Instant};

use polars::{
    io::SerReader,
    prelude::{JsonFormat, JsonReader},
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

pub struct JsonLineToDataFrame {
    ignore_errors: bool,
}

impl JsonLineToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_errors: args.ignore_errors,
        }
    }
}

impl Default for JsonLineToDataFrame {
    fn default() -> Self {
        Self {
            ignore_errors: true,
        }
    }
}

impl ReadToDataFrames for JsonLineToDataFrame {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let df = match &input {
            Source::File(path) => JsonReader::new(File::open(path)?)
                .with_json_format(JsonFormat::JsonLines)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .set_rechunk(true)
                .finish()?,
            Source::Stdin => JsonReader::new(stdin())
                .with_json_format(JsonFormat::JsonLines)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}

/// Streaming JSONL reader. Runs on its own thread, buffers lines up to
/// `config.batch_rows` or `config.batch_ms`, then parses each buffered chunk
/// with the existing `JsonReader` and emits it as a `StreamEvent::Batch`.
pub struct JsonLineStreamReader {
    ignore_errors: bool,
    config: StreamingConfig,
}

impl JsonLineStreamReader {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_errors: args.ignore_errors,
            config: StreamingConfig::from_args(args),
        }
    }
}

impl StreamToDataFrames for JsonLineStreamReader {
    fn stream_to_data_frames(self: Box<Self>, input: Source, sender: SyncSender<StreamEvent>) {
        let name = input.table_name();
        let ignore_errors = self.ignore_errors;
        let config = self.config;

        thread::spawn(move || {
            // Only stdin streaming is wired for now; file-backed streaming
            // would need a different read loop (tail-follow) that we are not
            // taking on as part of this change.
            if !matches!(input, Source::Stdin) {
                let _ = sender.send(StreamEvent::Error {
                    name: name.clone(),
                    error: anyhow::anyhow!("streaming JSONL only supports stdin"),
                });
                return;
            }

            let reader = BufReader::new(stdin_raw_locked());
            run_jsonl_stream(reader, &name, ignore_errors, config, &sender);
        });
    }
}

/// Core streaming loop, extracted so it can be unit-tested against any
/// `BufRead` source (a `Cursor` in tests, `StdinLock` in production).
pub(crate) fn run_jsonl_stream<R: BufRead>(
    reader: R,
    name: &str,
    ignore_errors: bool,
    config: StreamingConfig,
    sender: &SyncSender<StreamEvent>,
) {
    let mut pending: Vec<u8> = Vec::with_capacity(64 * 1024);
    let mut pending_rows: usize = 0;
    let mut schema_emitted = false;
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_millis(config.batch_ms);
    let mut lines = reader.lines();

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if !line.is_empty() {
                    pending.extend_from_slice(line.as_bytes());
                    pending.push(b'\n');
                    pending_rows += 1;
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
                if pending_rows > 0
                    && !flush_batch(
                        name,
                        &mut pending,
                        &mut pending_rows,
                        ignore_errors,
                        &mut schema_emitted,
                        sender,
                    )
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
        let size_ready = pending_rows >= config.batch_rows;
        if pending_rows > 0 && (time_ready || size_ready) {
            if !flush_batch(
                name,
                &mut pending,
                &mut pending_rows,
                ignore_errors,
                &mut schema_emitted,
                sender,
            ) {
                return;
            }
            last_flush = Instant::now();
        }
    }
}

/// Parse and send one batch. Returns `false` if the receiver has been dropped
/// or a fatal error was emitted (caller should exit the loop).
fn flush_batch(
    name: &str,
    pending: &mut Vec<u8>,
    pending_rows: &mut usize,
    ignore_errors: bool,
    schema_emitted: &mut bool,
    sender: &SyncSender<StreamEvent>,
) -> bool {
    let bytes = std::mem::take(pending);
    *pending_rows = 0;
    let df = match JsonReader::new(Cursor::new(bytes))
        .with_json_format(JsonFormat::JsonLines)
        .infer_schema_len(None)
        .with_ignore_errors(ignore_errors)
        .set_rechunk(true)
        .finish()
    {
        Ok(df) => df,
        Err(err) => {
            let _ = sender.send(StreamEvent::Error {
                name: name.to_string(),
                error: anyhow::anyhow!(err),
            });
            return false;
        }
    };

    if !*schema_emitted {
        let schema = std::sync::Arc::new(df.schema().as_ref().clone());
        if sender
            .send(StreamEvent::Schema {
                name: name.to_string(),
                schema,
            })
            .is_err()
        {
            return false;
        }
        *schema_emitted = true;
    }
    sender
        .send(StreamEvent::Batch {
            name: name.to_string(),
            rows: df,
        })
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::sync_channel;

    fn drain_events(rx: std::sync::mpsc::Receiver<StreamEvent>) -> Vec<StreamEvent> {
        let mut out = Vec::new();
        while let Ok(evt) = rx.recv() {
            out.push(evt);
        }
        out
    }

    fn config(batch_rows: usize, batch_ms: u64) -> StreamingConfig {
        StreamingConfig {
            batch_rows,
            batch_ms,
        }
    }

    #[test]
    fn single_batch_emits_schema_then_batch_then_eof() {
        let data = b"{\"a\":1}\n{\"a\":2}\n{\"a\":3}\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_jsonl_stream(Cursor::new(data), "t", true, config(1000, 10_000), &tx);
        drop(tx);
        let events = drain_events(rx);
        assert_eq!(events.len(), 3, "expected schema+batch+eof, got {events:?}");
        assert!(matches!(events[0], StreamEvent::Schema { .. }));
        match &events[1] {
            StreamEvent::Batch { rows, name } => {
                assert_eq!(name, "t");
                assert_eq!(rows.height(), 3);
                assert_eq!(rows.width(), 1);
            }
            other => panic!("expected batch, got {other:?}"),
        }
        assert!(matches!(events[2], StreamEvent::Eof { .. }));
    }

    #[test]
    fn schema_emitted_only_once_across_multiple_batches() {
        // Force two batches by setting batch_rows=2 with 4 lines of input.
        let data = b"{\"a\":1}\n{\"a\":2}\n{\"a\":3}\n{\"a\":4}\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_jsonl_stream(Cursor::new(data), "t", true, config(2, 10_000), &tx);
        drop(tx);
        let events = drain_events(rx);
        let schema_count = events
            .iter()
            .filter(|e| matches!(e, StreamEvent::Schema { .. }))
            .count();
        let batch_count = events
            .iter()
            .filter(|e| matches!(e, StreamEvent::Batch { .. }))
            .count();
        assert_eq!(schema_count, 1, "schema should be emitted exactly once");
        assert_eq!(batch_count, 2, "expected two batches, got {events:?}");
        assert!(matches!(events.last().unwrap(), StreamEvent::Eof { .. }));
    }

    #[test]
    fn empty_input_emits_only_eof() {
        let data: Vec<u8> = Vec::new();
        let (tx, rx) = sync_channel::<StreamEvent>(4);
        run_jsonl_stream(Cursor::new(data), "t", true, config(1000, 10_000), &tx);
        drop(tx);
        let events = drain_events(rx);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], StreamEvent::Eof { .. }));
    }

    #[test]
    fn blank_lines_are_skipped() {
        let data = b"{\"a\":1}\n\n{\"a\":2}\n\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_jsonl_stream(Cursor::new(data), "t", true, config(1000, 10_000), &tx);
        drop(tx);
        let events = drain_events(rx);
        let batch = events
            .iter()
            .find_map(|e| match e {
                StreamEvent::Batch { rows, .. } => Some(rows),
                _ => None,
            })
            .expect("expected a batch");
        assert_eq!(batch.height(), 2);
    }

    #[test]
    fn parse_error_emits_error_event_and_stops() {
        // ignore_errors=false so invalid JSON surfaces as StreamEvent::Error.
        let data = b"{not json}\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_jsonl_stream(Cursor::new(data), "t", false, config(1000, 10_000), &tx);
        drop(tx);
        let events = drain_events(rx);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, StreamEvent::Error { .. })),
            "expected Error event, got {events:?}"
        );
        // After an error we return without emitting Eof.
        assert!(
            !events.iter().any(|e| matches!(e, StreamEvent::Eof { .. })),
            "should not emit Eof after error, got {events:?}"
        );
    }
}
