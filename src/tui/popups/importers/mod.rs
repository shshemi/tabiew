use crate::{
    handler::message::Message,
    io::{Resource, reader::ReadToDataFrames},
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

fn dismiss_overlay_and_load_data_frame(source: Resource, rtdf: impl ReadToDataFrames) {
    Message::AppDismissOverlay.enqueue();
    match rtdf.read_to_data_frames(source.clone()) {
        Ok(named_frames) => {
            let count = named_frames.len();
            for (name, df) in named_frames {
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
