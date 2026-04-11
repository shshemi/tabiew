use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use polars::{
    frame::DataFrame,
    io::{SerReader, mmap::MmapBytesReader},
    prelude::{CsvParseOptions, CsvReadOptions, SchemaRef},
};

use crate::{
    AppResult,
    args::{Args, InferSchema},
    misc::{
        stdin::{stdin, stdin_raw_locked},
        type_ext::ToAscii,
    },
    reader::{
        NamedFrames, ReadToDataFrames, Source, StreamEvent, StreamToDataFrames,
        traits::StreamingConfig,
    },
};

pub struct CsvToDataFrame {
    infer_schema: InferSchema,
    quote_char: char,
    separator_char: char,
    no_header: bool,
    ignore_errors: bool,
    truncate_ragged_lines: bool,
}

impl CsvToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
            truncate_ragged_lines: args.truncate_ragged_lines,
        }
    }

    pub fn with_separator(mut self, c: char) -> Self {
        self.separator_char = c;
        self
    }

    pub fn with_no_header(mut self, no_header: bool) -> Self {
        self.no_header = no_header;
        self
    }

    pub fn with_quote_char(mut self, c: char) -> Self {
        self.quote_char = c;
        self
    }

    fn try_into_frame(&self, reader: impl MmapBytesReader) -> AppResult<DataFrame> {
        let df = CsvReadOptions::default()
            .with_ignore_errors(self.ignore_errors)
            .with_infer_schema_length(self.infer_schema.to_csv_infer_schema_length())
            .with_has_header(!self.no_header)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_truncate_ragged_lines(self.truncate_ragged_lines)
                    .with_quote_char(self.quote_char.to_ascii())
                    .with_separator(
                        self.separator_char
                            .to_ascii()
                            .ok_or(anyhow!("non-ASCII separator character"))?,
                    ),
            )
            .with_rechunk(true)
            .into_reader_with_file_handle(reader)
            .finish()?;
        Ok(df)
    }
}

impl Default for CsvToDataFrame {
    fn default() -> Self {
        Self {
            infer_schema: InferSchema::Safe,
            quote_char: '"',
            separator_char: ',',
            no_header: false,
            ignore_errors: true,
            truncate_ragged_lines: false,
        }
    }
}

impl ReadToDataFrames for CsvToDataFrame {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let df = match &input {
            Source::File(path) => self.try_into_frame(File::open(path)?),
            Source::Stdin => self.try_into_frame(stdin()),
        }?;
        Ok([(input.table_name(), df)].into())
    }
}

/// Streaming CSV/TSV/DSV reader. Reads stdin line by line, buffers lines up
/// to `config.batch_rows` or `config.batch_ms`, then parses each buffered
/// chunk using the existing `CsvReadOptions`. The header line (when present)
/// is captured on the first non-empty input line and prepended to every
/// parsed batch. After the first batch the schema is locked and passed via
/// `with_schema` so subsequent batches never re-infer types and drift.
pub struct CsvStreamReader {
    infer_schema: InferSchema,
    quote_char: char,
    separator_char: char,
    no_header: bool,
    ignore_errors: bool,
    truncate_ragged_lines: bool,
    config: StreamingConfig,
}

impl CsvStreamReader {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
            truncate_ragged_lines: args.truncate_ragged_lines,
            config: StreamingConfig::from_args(args),
        }
    }

    pub fn with_separator(mut self, c: char) -> Self {
        self.separator_char = c;
        self
    }
}

impl StreamToDataFrames for CsvStreamReader {
    fn stream_to_data_frames(self: Box<Self>, input: Source, sender: SyncSender<StreamEvent>) {
        let name = input.table_name();
        let opts = CsvReaderOpts {
            infer_schema: self.infer_schema,
            quote_char: self.quote_char,
            separator_char: self.separator_char,
            no_header: self.no_header,
            ignore_errors: self.ignore_errors,
            truncate_ragged_lines: self.truncate_ragged_lines,
        };
        let config = self.config;

        thread::spawn(move || {
            if !matches!(input, Source::Stdin) {
                let _ = sender.send(StreamEvent::Error {
                    name: name.clone(),
                    error: anyhow::anyhow!("streaming CSV only supports stdin"),
                });
                return;
            }
            let reader = BufReader::new(stdin_raw_locked());
            run_csv_stream(reader, &name, &opts, config, &sender);
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CsvReaderOpts {
    pub infer_schema: InferSchema,
    pub quote_char: char,
    pub separator_char: char,
    pub no_header: bool,
    pub ignore_errors: bool,
    pub truncate_ragged_lines: bool,
}

pub(crate) fn run_csv_stream<R: BufRead>(
    reader: R,
    name: &str,
    opts: &CsvReaderOpts,
    config: StreamingConfig,
    sender: &SyncSender<StreamEvent>,
) {
    let mut pending_bytes: Vec<u8> = Vec::with_capacity(64 * 1024);
    let mut pending_rows: usize = 0;
    let mut header_line: Option<Vec<u8>> = None;
    let mut locked_schema: Option<SchemaRef> = None;
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_millis(config.batch_ms);
    let mut lines = reader.lines();

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if line.is_empty() {
                    continue;
                }
                // Capture the header line once (when configured). We keep a
                // copy and never include it in a data batch.
                if !opts.no_header && header_line.is_none() {
                    header_line = Some(line.into_bytes());
                    continue;
                }
                pending_bytes.extend_from_slice(line.as_bytes());
                pending_bytes.push(b'\n');
                pending_rows += 1;
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
                    && !flush_csv_batch(
                        name,
                        &mut pending_bytes,
                        &mut pending_rows,
                        header_line.as_deref(),
                        &mut locked_schema,
                        opts,
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
            if !flush_csv_batch(
                name,
                &mut pending_bytes,
                &mut pending_rows,
                header_line.as_deref(),
                &mut locked_schema,
                opts,
                sender,
            ) {
                return;
            }
            last_flush = Instant::now();
        }
    }
}

fn flush_csv_batch(
    name: &str,
    pending: &mut Vec<u8>,
    pending_rows: &mut usize,
    header_line: Option<&[u8]>,
    locked_schema: &mut Option<SchemaRef>,
    opts: &CsvReaderOpts,
    sender: &SyncSender<StreamEvent>,
) -> bool {
    // Prepend the captured header line (if any) so every parsed batch
    // agrees on column count and ordering.
    let body = std::mem::take(pending);
    *pending_rows = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(header_line.map_or(0, |h| h.len() + 1) + body.len());
    if let Some(h) = header_line {
        buf.extend_from_slice(h);
        buf.push(b'\n');
    }
    buf.extend_from_slice(&body);

    let separator = match opts.separator_char.to_ascii() {
        Some(b) => b,
        None => {
            let _ = sender.send(StreamEvent::Error {
                name: name.to_string(),
                error: anyhow::anyhow!("non-ASCII separator character"),
            });
            return false;
        }
    };
    let parse_opts = CsvParseOptions::default()
        .with_truncate_ragged_lines(opts.truncate_ragged_lines)
        .with_quote_char(opts.quote_char.to_ascii())
        .with_separator(separator);

    let read_opts = CsvReadOptions::default()
        .with_ignore_errors(opts.ignore_errors)
        .with_infer_schema_length(opts.infer_schema.to_csv_infer_schema_length())
        .with_has_header(!opts.no_header)
        .with_schema(locked_schema.clone())
        .with_parse_options(parse_opts)
        .with_rechunk(true);

    let df = match read_opts
        .into_reader_with_file_handle(Cursor::new(buf))
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

    // Lock the schema after the first successful batch and emit it so the
    // UI can render the header row immediately.
    if locked_schema.is_none() {
        let schema: Arc<polars::prelude::Schema> = Arc::new(df.schema().as_ref().clone());
        locked_schema.replace(schema.clone());
        if sender
            .send(StreamEvent::Schema {
                name: name.to_string(),
                schema,
            })
            .is_err()
        {
            return false;
        }
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

    fn default_opts() -> CsvReaderOpts {
        CsvReaderOpts {
            infer_schema: InferSchema::Safe,
            quote_char: '"',
            separator_char: ',',
            no_header: false,
            ignore_errors: true,
            truncate_ragged_lines: false,
        }
    }

    fn config(batch_rows: usize, batch_ms: u64) -> StreamingConfig {
        StreamingConfig {
            batch_rows,
            batch_ms,
        }
    }

    fn drain(rx: std::sync::mpsc::Receiver<StreamEvent>) -> Vec<StreamEvent> {
        let mut out = Vec::new();
        while let Ok(evt) = rx.recv() {
            out.push(evt);
        }
        out
    }

    #[test]
    fn single_batch_with_header() {
        let data = b"id,v\n1,a\n2,b\n3,c\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_csv_stream(
            Cursor::new(data),
            "t",
            &default_opts(),
            config(1000, 10_000),
            &tx,
        );
        drop(tx);
        let events = drain(rx);
        assert_eq!(events.len(), 3);
        match &events[1] {
            StreamEvent::Batch { rows, .. } => {
                assert_eq!(rows.height(), 3);
                assert_eq!(rows.get_column_names(), &["id", "v"]);
            }
            other => panic!("expected batch, got {other:?}"),
        }
    }

    #[test]
    fn schema_locked_across_batches_prevents_drift() {
        // Batch 1 has integers in column `n`. Batch 2 would normally be
        // inferred as string due to "abc", but the locked schema forces an
        // integer parse, which fails — with ignore_errors the row becomes
        // null rather than breaking the stream.
        let data = b"n\n1\n2\n3\n4\nabc\n5\n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        run_csv_stream(
            Cursor::new(data),
            "t",
            &default_opts(),
            config(3, 10_000),
            &tx,
        );
        drop(tx);
        let events = drain(rx);
        let mut batches = events.iter().filter_map(|e| match e {
            StreamEvent::Batch { rows, .. } => Some(rows),
            _ => None,
        });
        let b1 = batches.next().expect("first batch");
        let b2 = batches.next().expect("second batch");
        assert_eq!(b1.schema(), b2.schema(), "schema must be locked");
    }
}
