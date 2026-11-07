use std::io::{Cursor, Read};

use anyhow::{Context, anyhow};
use arrow_array::{RecordBatch, cast::AsArray};
use arrow_ipc::writer::FileWriter;
use polars::{io::SerReader, prelude::IpcReader};
use vortex::io::session::RuntimeSessionExt;
use vortex::{
    VortexSessionDefault,
    array::VortexSessionExecute,
    arrow::ArrowSessionExt,
    dtype::DType,
    file::OpenOptionsSessionExt,
    io::runtime::{BlockingRuntime, single::SingleThreadRuntime},
    session::VortexSession,
};

use crate::{
    AppResult,
    args::Args,
    misc::stdin::stdin,
    readers::{DataFrameReader, NamedFrames, ReaderSource},
};

#[derive(Debug, Default)]
pub struct VortexToDataFrame {
    max_rows: Option<usize>,
}

impl VortexToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            max_rows: args.max_rows,
        }
    }

    pub fn with_max_rows(mut self, max_rows: Option<usize>) -> Self {
        self.max_rows = max_rows;
        self
    }
}

impl DataFrameReader for VortexToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let runtime = SingleThreadRuntime::default();
        let session = VortexSession::default().with_handle(runtime.handle());

        let file = match &input {
            ReaderSource::File(path) => runtime
                .block_on(session.open_options().open_path(path))
                .with_context(|| format!("Failed to open Vortex file '{}'", path.display()))?,
            ReaderSource::Stdin => {
                // Vortex keeps its footer at the end of the file, so it needs random access.
                let mut buf = Vec::new();
                stdin().read_to_end(&mut buf)?;
                session.open_options().open_buffer(buf)?
            }
        };

        if !matches!(file.dtype(), DType::Struct(..)) {
            return Err(anyhow!(
                "Vortex file has a non-tabular top-level type ({}); only struct (table) files are supported",
                file.dtype()
            ));
        }

        let iter = file
            .scan()?
            .with_some_limit(self.max_rows.map(|n| n as u64))
            .into_array_iter(&runtime)?;
        let fallback_schema = session.arrow().to_arrow_schema(file.dtype())?;

        let mut ctx = session.create_execution_ctx();
        let mut batches: Vec<RecordBatch> = Vec::new();
        for array in iter {
            let arrow = session.arrow().execute_arrow(array?, None, &mut ctx)?;
            batches.push(RecordBatch::from(arrow.as_struct()));
        }

        // Polars and arrow-rs use different Arrow implementations, so hand the data over
        // through an in-memory Arrow IPC file.
        let schema = batches
            .first()
            .map(RecordBatch::schema)
            .unwrap_or_else(|| fallback_schema.into());
        let mut ipc = Cursor::new(Vec::new());
        {
            let mut writer = FileWriter::try_new(&mut ipc, &schema)?;
            for batch in &batches {
                writer.write(batch)?;
            }
            writer.finish()?;
        }
        drop(batches);
        ipc.set_position(0);

        let df = IpcReader::new(ipc).set_rechunk(true).finish()?;
        Ok([(input.table_name(), df)].into())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use polars::prelude::{AnyValue, DataType, TimeUnit};

    use super::*;

    fn fixture(name: &str) -> ReaderSource {
        ReaderSource::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src/readers/testdata")
                .join(name),
        )
    }

    fn read(reader: VortexToDataFrame, name: &str) -> polars::frame::DataFrame {
        let frames = reader.read_to_data_frames(fixture(name)).unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].0, name.trim_end_matches(".vortex"));
        frames.into_iter().next().unwrap().1
    }

    #[test]
    fn reads_all_rows_and_columns() {
        let df = read(VortexToDataFrame::default(), "sample.vortex");
        assert_eq!(df.shape(), (100, 9));
        let names: Vec<_> = df.get_column_names().iter().map(|n| n.as_str()).collect();
        assert_eq!(names, ["id", "f", "s", "b", "d", "ts", "dec", "lst", "st"]);

        assert_eq!(df.column("id").unwrap().dtype(), &DataType::Int64);
        assert_eq!(df.column("f").unwrap().dtype(), &DataType::Float64);
        assert_eq!(df.column("b").unwrap().dtype(), &DataType::Boolean);
        assert_eq!(df.column("d").unwrap().dtype(), &DataType::Date);
        assert!(matches!(
            df.column("ts").unwrap().dtype(),
            DataType::Datetime(TimeUnit::Microseconds, _)
        ));
        assert!(matches!(
            df.column("dec").unwrap().dtype(),
            DataType::Decimal(..)
        ));
        assert!(matches!(
            df.column("lst").unwrap().dtype(),
            DataType::List(_)
        ));
        assert!(matches!(
            df.column("st").unwrap().dtype(),
            DataType::Struct(_)
        ));
    }

    #[test]
    fn preserves_values_and_nulls() {
        let df = read(VortexToDataFrame::default(), "sample.vortex");
        let id = df.column("id").unwrap().as_materialized_series();
        assert_eq!(id.get(99).unwrap(), AnyValue::Int64(99));

        // f is null where i % 7 == 0, s is null where i % 5 == 0.
        let f = df.column("f").unwrap();
        assert_eq!(f.null_count(), 15);
        assert!(f.get(7).unwrap().is_null());
        assert_eq!(f.get(8).unwrap(), AnyValue::Float64(4.0));
        let s = df.column("s").unwrap();
        assert_eq!(s.null_count(), 20);
        assert_eq!(s.get(3).unwrap(), AnyValue::String("row-3"));
        assert!(s.get(10).unwrap().is_null());
    }

    #[test]
    fn max_rows_limits_output() {
        let df = read(
            VortexToDataFrame::default().with_max_rows(Some(10)),
            "sample.vortex",
        );
        assert_eq!(df.height(), 10);
        assert_eq!(df.width(), 9);
        let df = read(
            VortexToDataFrame::default().with_max_rows(Some(0)),
            "sample.vortex",
        );
        assert_eq!(df.height(), 0);
        let df = read(
            VortexToDataFrame::default().with_max_rows(Some(1000)),
            "sample.vortex",
        );
        assert_eq!(df.height(), 100);
    }

    #[test]
    fn empty_file_keeps_schema() {
        let df = read(VortexToDataFrame::default(), "empty.vortex");
        assert_eq!(df.height(), 0);
        assert_eq!(df.width(), 9);
    }

    #[test]
    fn rejects_non_vortex_input() {
        let err = VortexToDataFrame::default()
            .read_to_data_frames(ReaderSource::File(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
            ))
            .unwrap_err();
        assert!(err.to_string().contains("Vortex"), "{err:#}");
    }

    #[test]
    fn missing_file_errors() {
        assert!(
            VortexToDataFrame::default()
                .read_to_data_frames(fixture("nope.vortex"))
                .is_err()
        );
    }
}
