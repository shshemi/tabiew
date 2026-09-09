use std::sync::Arc;

use crate::{
    handler::message::Message,
    io::{DataSource, reader::ReaderSource},
    misc::{
        file_identity::FileIdentity,
        refresh::RefreshSource,
        remote_load,
        sql::{TableSource, sql},
    },
};

pub mod arrow;
pub mod avro;
pub mod csv;
pub mod excel;
pub mod fwf;
pub mod html;
mod import_source_picker;
pub mod json;
pub mod jsonl;
pub mod logfmt;
pub mod markdown;
pub mod parquet;
pub mod sqlite;
pub mod tsv;

fn dismiss_overlay_and_load_data_frame(source: DataSource, reader: impl remote_load::Reader) {
    Message::AppDismissOverlay.enqueue();
    let reader: Arc<dyn remote_load::Reader> = Arc::new(reader);
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
            // Captured before the read so that a refresh compares against the file that was
            // actually imported. Sources that are not regular files are simply not refreshable.
            let identity = FileIdentity::capture(&path_buf).ok();
            let frames = match reader.read_to_data_frames(ReaderSource::File(path_buf.clone())) {
                Ok(f) => f,
                Err(err) => {
                    Message::AppShowError(err.to_string()).enqueue();
                    return;
                }
            };
            let count = frames.len();
            for (frame_index, (name, df)) in frames.into_iter().enumerate() {
                let name = match identity.clone() {
                    Some(identity) => sql().register_refreshable(
                        &name,
                        df.clone(),
                        TableSource::File(path_buf.clone()),
                        RefreshSource::new(reader.clone(), identity, &name, frame_index, count),
                    ),
                    None => sql().register(&name, df.clone(), TableSource::File(path_buf.clone())),
                };
                Message::TabsAddNamePane(df, name).enqueue();
            }
            Message::AppShowToast(format!(
                "{} data frame(s) were imported from {}",
                count,
                path_buf.to_string_lossy()
            ))
            .enqueue();
        }
        DataSource::Url(url) => Message::AppDownloadDataSource(url, reader).enqueue(),
    };
}
