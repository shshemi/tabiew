use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{self, BufRead, BufReader, Cursor, Read},
    iter::once,
    sync::Arc,
    sync::mpsc::SyncSender,
    thread,
    time::{Duration, Instant},
};

use fwf_rs::Reader;
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{Column, Schema},
};

use crate::{
    AppResult,
    args::Args,
    misc::{
        iter_ext::ZipItersExt, stdin::stdin_raw_locked, table_name_generator::TableNameGeneratorExt,
    },
    reader::{StreamEvent, StreamToDataFrames, traits::StreamingConfig},
};

use super::{NamedFrames, ReadToDataFrames, Source};

pub struct FwfToDataFrame {
    widths: Vec<usize>,
    has_header: bool,
    separator_length: usize,
    flexible_width: bool,
}

impl FwfToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            widths: parse_width(&args.widths).unwrap_or_default(),
            has_header: !args.no_header,
            separator_length: args.separator_length,
            flexible_width: !args.no_flexible_width,
        }
    }

    pub fn with_widths(mut self, widths: Vec<usize>) -> Self {
        self.widths = widths;
        self
    }

    pub fn with_has_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    pub fn with_separator_length(mut self, separator_length: usize) -> Self {
        self.separator_length = separator_length;
        self
    }

    pub fn with_flexible_width(mut self, flexible_width: bool) -> Self {
        self.flexible_width = flexible_width;
        self
    }
}

impl Default for FwfToDataFrame {
    fn default() -> Self {
        Self {
            widths: Vec::default(),
            has_header: true,
            separator_length: 0,
            flexible_width: true,
        }
    }
}

impl ReadToDataFrames for FwfToDataFrame {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let file_content = match &input {
            Source::File(path) => read_to_string(path)?,
            Source::Stdin => {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf)?;
                buf
            }
        };

        let widths = if self.widths.is_empty() {
            let common_space_indices = file_content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let length = line.chars().count();
                    let spaces = line
                        .chars()
                        .enumerate()
                        .filter_map(|(i, c)| c.is_whitespace().then_some(i))
                        .collect::<HashSet<usize>>();
                    (length, spaces)
                })
                .reduce(|(la, sa), (lb, sb)| (la.max(lb), sa.intersection(&sb).copied().collect()))
                .map(|(len, idx_set)| idx_set.into_iter().chain(once(len)).sorted().collect_vec())
                .unwrap_or_default();
            infer_widths(common_space_indices)
        } else {
            self.widths.clone()
        };
        let reader = Reader::new(
            Cursor::new(file_content),
            widths.clone(),
            self.separator_length,
            self.flexible_width,
            self.has_header,
        )?;
        let header = reader
            .header()
            .map(|rec| {
                rec.iter().fold(Vec::new(), |mut vec, slice| {
                    if let Some(name) = slice.snake_case_names().find(|name| !vec.contains(name)) {
                        vec.push(name);
                    } else {
                        vec.push(format!("column_{}", vec.len() + 1));
                    }
                    vec
                })
            })
            .unwrap_or_else(|| {
                (0..widths.len())
                    .map(|idx| format!("column_{}", idx + 1))
                    .collect_vec()
            });

        let columns = reader
            .records()
            .filter_map(Result::ok)
            .map(|record| {
                record
                    .iter()
                    .map(str::trim)
                    .map(ToOwned::to_owned)
                    .collect_vec()
                    .into_iter()
            })
            .zip_iters()
            .collect_vec();

        let df = DataFrame::new_infer_height(
            header
                .into_iter()
                .zip(columns)
                .map(|(name, vals)| Column::new(name.into(), vals))
                .collect(),
        )?;

        Ok([(input.table_name(), df)].into())
    }
}

/// Streaming FWF reader. Requires `--widths` to be set because width
/// inference needs the whole file up front. Each batch is parsed with a
/// fixed layout, headers are captured from the first batch, and the schema
/// is locked (all columns are String so drift is impossible).
pub struct FwfStreamReader {
    widths: Vec<usize>,
    has_header: bool,
    separator_length: usize,
    flexible_width: bool,
    config: StreamingConfig,
}

impl FwfStreamReader {
    pub fn from_args(args: &Args) -> Self {
        Self {
            widths: parse_width(&args.widths).unwrap_or_default(),
            has_header: !args.no_header,
            separator_length: args.separator_length,
            flexible_width: !args.no_flexible_width,
            config: StreamingConfig::from_args(args),
        }
    }
}

impl StreamToDataFrames for FwfStreamReader {
    fn stream_to_data_frames(self: Box<Self>, input: Source, sender: SyncSender<StreamEvent>) {
        let name = input.table_name();
        let opts = FwfReaderOpts {
            widths: self.widths.clone(),
            has_header: self.has_header,
            separator_length: self.separator_length,
            flexible_width: self.flexible_width,
        };
        let config = self.config;
        thread::spawn(move || {
            if !matches!(input, Source::Stdin) {
                let _ = sender.send(StreamEvent::Error {
                    name: name.clone(),
                    error: anyhow::anyhow!("streaming FWF only supports stdin"),
                });
                return;
            }
            if opts.widths.is_empty() {
                let _ = sender.send(StreamEvent::Error {
                    name: name.clone(),
                    error: anyhow::anyhow!(
                        "streaming FWF requires --widths; width inference needs the full file"
                    ),
                });
                return;
            }
            let reader = BufReader::new(stdin_raw_locked());
            run_fwf_stream(reader, &name, &opts, config, &sender);
        });
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FwfReaderOpts {
    pub widths: Vec<usize>,
    pub has_header: bool,
    pub separator_length: usize,
    pub flexible_width: bool,
}

pub(crate) fn run_fwf_stream<R: BufRead>(
    reader: R,
    name: &str,
    opts: &FwfReaderOpts,
    config: StreamingConfig,
    sender: &SyncSender<StreamEvent>,
) {
    let mut pending: Vec<String> = Vec::new();
    let mut header_line: Option<String> = None;
    let mut locked_headers: Option<Vec<String>> = None;
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_millis(config.batch_ms);
    let mut lines = reader.lines();

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if line.is_empty() {
                    continue;
                }
                if opts.has_header && header_line.is_none() {
                    header_line = Some(line);
                    continue;
                }
                pending.push(line);
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
                    && !flush_fwf_batch(
                        name,
                        &mut pending,
                        header_line.as_deref(),
                        &mut locked_headers,
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
        let size_ready = pending.len() >= config.batch_rows;
        if !pending.is_empty()
            && (time_ready || size_ready)
            && !flush_fwf_batch(
                name,
                &mut pending,
                header_line.as_deref(),
                &mut locked_headers,
                opts,
                sender,
            )
        {
            return;
        }
        if time_ready || size_ready {
            last_flush = Instant::now();
        }
    }
}

fn flush_fwf_batch(
    name: &str,
    pending: &mut Vec<String>,
    header_line: Option<&str>,
    locked_headers: &mut Option<Vec<String>>,
    opts: &FwfReaderOpts,
    sender: &SyncSender<StreamEvent>,
) -> bool {
    let rows = std::mem::take(pending);
    // Prepend the captured header line (if any) so the fwf Reader can emit
    // its record-iterator with consistent positions.
    let mut body = String::new();
    if let Some(h) = header_line {
        body.push_str(h);
        body.push('\n');
    }
    for r in &rows {
        body.push_str(r);
        body.push('\n');
    }

    let reader = match Reader::new(
        Cursor::new(body),
        opts.widths.clone(),
        opts.separator_length,
        opts.flexible_width,
        opts.has_header,
    ) {
        Ok(r) => r,
        Err(err) => {
            let _ = sender.send(StreamEvent::Error {
                name: name.to_string(),
                error: anyhow::anyhow!(err),
            });
            return false;
        }
    };

    let headers = if let Some(locked) = locked_headers.as_ref() {
        locked.clone()
    } else {
        reader
            .header()
            .map(|rec| {
                rec.iter().fold(Vec::new(), |mut vec, slice| {
                    if let Some(name) = slice.snake_case_names().find(|name| !vec.contains(name)) {
                        vec.push(name);
                    } else {
                        vec.push(format!("column_{}", vec.len() + 1));
                    }
                    vec
                })
            })
            .unwrap_or_else(|| {
                (0..opts.widths.len())
                    .map(|idx| format!("column_{}", idx + 1))
                    .collect_vec()
            })
    };

    let columns = reader
        .records()
        .filter_map(Result::ok)
        .map(|record| {
            record
                .iter()
                .map(str::trim)
                .map(ToOwned::to_owned)
                .collect_vec()
                .into_iter()
        })
        .zip_iters()
        .collect_vec();

    // zip_iters truncates to the shortest row, so if no data rows made it
    // through we need to emit empty-but-named columns to keep the schema.
    let columns = if columns.is_empty() {
        headers.iter().map(|_| Vec::<String>::new()).collect_vec()
    } else {
        columns
    };

    let df = match DataFrame::new_infer_height(
        headers
            .iter()
            .cloned()
            .zip(columns)
            .map(|(name, vals)| Column::new(name.into(), vals))
            .collect(),
    ) {
        Ok(df) => df,
        Err(err) => {
            let _ = sender.send(StreamEvent::Error {
                name: name.to_string(),
                error: anyhow::anyhow!(err),
            });
            return false;
        }
    };

    if locked_headers.is_none() {
        *locked_headers = Some(headers);
        let schema: Arc<Schema> = Arc::new(df.schema().as_ref().clone());
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

fn parse_width(widths: impl AsRef<str>) -> AppResult<Vec<usize>> {
    Ok(widths
        .as_ref()
        .split(',')
        .map(|w| w.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn infer_widths(space_indices: Vec<usize>) -> Vec<usize> {
    let mut indices = Vec::default();
    let mut start = 0;
    for (i, idx) in space_indices.iter().enumerate() {
        if let Some(nidx) = space_indices.get(i + 1) {
            if nidx - idx > 1 {
                indices.push(idx - start);
                start = idx + 1
            }
        } else {
            indices.push(idx - start);
        }
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::sync_channel;

    fn drain(rx: std::sync::mpsc::Receiver<StreamEvent>) -> Vec<StreamEvent> {
        let mut out = Vec::new();
        while let Ok(evt) = rx.recv() {
            out.push(evt);
        }
        out
    }

    #[test]
    fn streams_fixed_width_rows_across_multiple_batches() {
        // 10-char name, 5-char id, no separator between columns.
        let data = b"name      id   \nfoo       1    \nbar       2    \nbaz       3    \nqux       4    \n".to_vec();
        let (tx, rx) = sync_channel::<StreamEvent>(16);
        let opts = FwfReaderOpts {
            widths: vec![10, 5],
            has_header: true,
            separator_length: 0,
            flexible_width: true,
        };
        run_fwf_stream(
            Cursor::new(data),
            "t",
            &opts,
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
            assert_eq!(b.width(), 2);
        }
        // Every row should land across the two batches.
        let total: usize = batches.iter().map(|b| b.height()).sum();
        assert_eq!(total, 4);
    }
}
