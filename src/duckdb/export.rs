use crate::{
    AppResult,
    duckdb::{DuckDB, ingester::progress::StepProgress, sql::quote},
    misc::osc52::CopyToClipboardOsc52,
    writers::{Destination, ExportFormat, escape_cell},
};
use anyhow::anyhow;
use duckdb::{
    Connection,
    arrow::util::display::{ArrayFormatter, FormatOptions},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::oneshot::{self, Receiver, error::TryRecvError};

pub(super) struct Exporting {
    pub(super) title: String,
    pub(super) progress: Arc<StepProgress>,
    done: String,
    clipboard: Option<PathBuf>,
    result: Receiver<AppResult<()>>,
}

impl DuckDB {
    pub fn export(&mut self, view: &str, format: ExportFormat, dest: Destination) -> AppResult<()> {
        let conn = self.session.session()?;
        conn.set_query_progress_tracking(true)?;
        let progress = StepProgress::new(1);
        progress.start(&conn);
        self.results += 1;
        let (path, clipboard) = match dest {
            Destination::File(path) => (path, None),
            Destination::Clipboard => {
                let path = self._scratch.dir().join(format!("export-{}", self.results));
                (path.clone(), Some(path))
            }
        };
        let name = format!(
            "'{}'",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        let target = match clipboard {
            Some(_) => "clipboard",
            None => name.as_str(),
        };
        let title = format!("Exporting to {target}");
        let done = format!("Exported to {target} in {} format", label(format));
        let view = view.to_string();
        let (sender, result) = oneshot::channel();
        std::thread::spawn(move || sender.send(write(&conn, &view, format, &path)));
        self.exporting.push(Exporting {
            title,
            progress,
            done,
            clipboard,
            result,
        });
        Ok(())
    }
    pub fn export_now(&self, view: &str, format: ExportFormat, path: &Path) -> AppResult<()> {
        write(&*self.session.session()?, view, format, path)
    }
    pub(super) fn poll_exports(&mut self) -> Vec<Result<String, String>> {
        let mut finished = Vec::new();
        let mut i = 0;
        while i < self.exporting.len() {
            let result = match self.exporting[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Export stopped without a result")),
            };
            let exporting = self.exporting.swap_remove(i);
            let result = result.and_then(|()| match &exporting.clipboard {
                Some(path) => {
                    let bytes = std::fs::read(path);
                    let _ = std::fs::remove_file(path);
                    bytes?.copy_to_clipboard_via_osc52();
                    Ok(())
                }
                None => Ok(()),
            });
            finished.push(
                result
                    .map(|()| exporting.done)
                    .map_err(|e| format!("{}: {e}", exporting.title)),
            );
        }
        finished
    }
}

fn label(format: ExportFormat) -> &'static str {
    match format {
        ExportFormat::Csv {
            separator: '\t', ..
        } => "TSV",
        ExportFormat::Csv { .. } => "CSV",
        ExportFormat::Json => "JSON",
        ExportFormat::Jsonl => "JSONL",
        ExportFormat::Parquet => "Parquet",
        ExportFormat::Arrow => "Arrow",
        ExportFormat::Avro => "Avro",
        ExportFormat::Markdown => "Markdown",
    }
}

fn literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn write(conn: &Connection, view: &str, format: ExportFormat, path: &Path) -> AppResult<()> {
    let copy = |options: String| {
        conn.execute_batch(&format!(
            "COPY (SELECT * FROM {}) TO {} ({options})",
            quote(view),
            literal(&path.to_string_lossy())
        ))
    };
    match format {
        ExportFormat::Csv {
            separator,
            quote,
            header,
        } => copy(format!(
            "FORMAT csv, DELIMITER {}, QUOTE {}, HEADER {header}",
            literal(&separator.to_string()),
            literal(&quote.to_string())
        ))?,
        ExportFormat::Json => copy("FORMAT json, ARRAY true".to_string())?,
        ExportFormat::Jsonl => copy("FORMAT json".to_string())?,
        ExportFormat::Parquet => copy("FORMAT parquet, COMPRESSION zstd".to_string())?,
        ExportFormat::Arrow | ExportFormat::Avro | ExportFormat::Markdown => {
            let mut statement = conn.prepare(&format!("SELECT * FROM {}", quote(view)))?;
            let batches = statement.query_arrow([])?;
            let schema = batches.get_schema();
            let mut file = BufWriter::new(File::create(path)?);
            match format {
                ExportFormat::Arrow => {
                    let mut writer = arrow_ipc::writer::FileWriter::try_new(file, &schema)?;
                    for batch in batches {
                        writer.write(&batch)?;
                    }
                    writer.finish()?;
                }
                ExportFormat::Avro => {
                    let mut writer =
                        arrow_avro::writer::AvroWriter::new(file, schema.as_ref().clone())?;
                    for batch in batches {
                        writer.write(&batch)?;
                    }
                    writer.finish()?;
                }
                _ => {
                    let header = schema
                        .fields()
                        .iter()
                        .map(|field| format!(" {} |", escape_cell(field.name())))
                        .collect::<String>();
                    writeln!(file, "|{header}")?;
                    writeln!(file, "|{}", "---|".repeat(schema.fields().len()))?;
                    let options = FormatOptions::default().with_null("null");
                    for batch in batches {
                        let columns = batch
                            .columns()
                            .iter()
                            .map(|column| ArrayFormatter::try_new(column.as_ref(), &options))
                            .collect::<Result<Vec<_>, _>>()?;
                        for row in 0..batch.num_rows() {
                            let mut line = String::from("|");
                            for column in &columns {
                                line.push(' ');
                                line.push_str(&escape_cell(&column.value(row).to_string()));
                                line.push_str(" |");
                            }
                            writeln!(file, "{line}")?;
                        }
                    }
                    file.flush()?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        duckdb::{DuckDBConfig, Opened, Store},
        reader::RowSource,
    };
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    #[test]
    fn every_format_is_written_from_a_view() {
        let dir = TempDir::new().unwrap();
        let mut db = DuckDB::new(DuckDBConfig {
            cache_dir: dir.path().join("cache"),
            ..DuckDBConfig::default()
        })
        .unwrap();
        let Opened::Ready(reader) = db
            .open_query(
                "SELECT * FROM (VALUES (1, 'a|b', 1.5), (2, NULL, 77.0)) t(id, note, x)",
                None,
                Store::View,
            )
            .unwrap()
        else {
            panic!("pending");
        };
        let view = reader.source().view().unwrap().to_string();
        let formats = [
            (
                "csv",
                ExportFormat::Csv {
                    separator: ',',
                    quote: '"',
                    header: true,
                },
            ),
            (
                "tsv",
                ExportFormat::Csv {
                    separator: '\t',
                    quote: '"',
                    header: false,
                },
            ),
            ("json", ExportFormat::Json),
            ("jsonl", ExportFormat::Jsonl),
            ("parquet", ExportFormat::Parquet),
            ("arrow", ExportFormat::Arrow),
            ("avro", ExportFormat::Avro),
            ("md", ExportFormat::Markdown),
        ];
        for (extension, format) in formats {
            let path = dir.path().join(format!("out.{extension}"));
            db.export(&view, format, Destination::File(path)).unwrap();
        }
        db.export(
            "missing",
            ExportFormat::Json,
            Destination::File(dir.path().join("no.json")),
        )
        .unwrap();
        let start = Instant::now();
        let mut finished = Vec::new();
        while finished.len() < formats.len() + 1 {
            assert!(start.elapsed() < Duration::from_secs(10));
            finished.extend(db.poll().unwrap().exports);
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(finished.iter().filter(|result| result.is_err()).count(), 1);
        assert!(finished.contains(&Ok("Exported to 'out.tsv' in TSV format".to_string())));
        let text = |extension: &str| {
            std::fs::read_to_string(dir.path().join(format!("out.{extension}"))).unwrap()
        };
        assert_eq!(text("csv"), "id,note,x\n1,a|b,1.5\n2,,77.0\n");
        assert_eq!(text("tsv"), "1\ta|b\t1.5\n2\t\t77.0\n");
        assert_eq!(
            text("jsonl"),
            "{\"id\":1,\"note\":\"a|b\",\"x\":1.5}\n{\"id\":2,\"note\":null,\"x\":77.0}\n"
        );
        assert_eq!(
            text("md"),
            "| id | note | x |\n|---|---|---|\n| 1 | a\\|b | 1.5 |\n| 2 | null | 77.0 |\n"
        );
        let conn = db.session.session().unwrap();
        for extension in ["json", "parquet", "csv"] {
            let rows: i64 = conn
                .query_row(
                    &format!(
                        "SELECT count(*) FROM {}",
                        literal(
                            &dir.path()
                                .join(format!("out.{extension}"))
                                .to_string_lossy()
                        )
                    ),
                    [],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(rows, 2, "{extension}");
        }
        let arrow = arrow_ipc::reader::FileReader::try_new(
            File::open(dir.path().join("out.arrow")).unwrap(),
            None,
        )
        .unwrap();
        assert_eq!(
            arrow.map(|batch| batch.unwrap().num_rows()).sum::<usize>(),
            2
        );
        let avro = arrow_avro::reader::ReaderBuilder::new()
            .build(std::io::BufReader::new(
                File::open(dir.path().join("out.avro")).unwrap(),
            ))
            .unwrap();
        assert_eq!(
            avro.map(|batch| batch.unwrap().num_rows()).sum::<usize>(),
            2
        );
    }
}
