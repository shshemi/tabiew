use crate::{
    handler::message::Message,
    io::{
        DataSource,
        reader::{DataFrameReader, ReaderSource},
    },
    misc::sql::{TableSource, sql},
};

pub mod arrow;
pub mod csv;
pub mod excel;
pub mod fwf;
mod import_source_picker;
pub mod json;
pub mod jsonl;
pub mod logfmt;
pub mod parquet;
pub mod sqlite;
pub mod tsv;

fn dismiss_overlay_and_load_data_frame(source: DataSource, reader: impl DataFrameReader) {
    Message::AppDismissOverlay.enqueue();
    match source {
        DataSource::Stdin => {
            let frames = match reader.read_to_data_frames(ReaderSource::Stdin) {
                Ok(f) => f,
                Err(err) => {
                    Message::AppShowError(err.to_string()).enqueue();
                    return;
                }
            };
            let count = frames.len();
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), TableSource::Stdin);
                Message::TabsAddNamePane(df, name).enqueue();
            }
            Message::AppShowToast(format!("{} data frame(s) were imported from Stdin", count,))
                .enqueue();
        }
        DataSource::File(path_buf) => {
            let frames = match reader.read_to_data_frames(ReaderSource::Stdin) {
                Ok(f) => f,
                Err(err) => {
                    Message::AppShowError(err.to_string()).enqueue();
                    return;
                }
            };
            let count = frames.len();
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), TableSource::Stdin);
                Message::TabsAddNamePane(df, name).enqueue();
            }
            Message::AppShowToast(format!(
                "{} data frame(s) were imported from {}",
                count,
                path_buf.to_string_lossy()
            ))
            .enqueue();
        }
        DataSource::Url(url) => Message::AppDownloadDataSource(url).enqueue(),
    };
}
