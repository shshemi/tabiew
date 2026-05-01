use crate::{
    handler::message::Message,
    io::{
        DataSource,
        reader::{DataFrameReader, ReaderSource},
    },
    misc::sql::sql,
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
    let frames = match &source {
        DataSource::Stdin => reader.read_to_data_frames(ReaderSource::Stdin),
        DataSource::File(path_buf) => {
            reader.read_to_data_frames(ReaderSource::File(path_buf.clone()))
        }
        DataSource::Url(_) => todo!(),
    };
    match frames {
        Ok(frames) => {
            let count = frames.len();
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                Message::TabsAddNamePane(df, name).enqueue();
            }
            Message::AppShowToast(format!(
                "{} data frame(s) were imported from {}",
                count,
                source.display_path()
            ))
            .enqueue();
        }
        Err(err) => Message::AppShowError(err.to_string()).enqueue(),
    }
}
