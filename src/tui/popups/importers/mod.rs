use crate::{
    handler::message::Message,
    misc::globals::sql,
    reader::{ReadToDataFrames, Source},
};

pub mod arrow;
pub mod csv;
pub mod excel;
pub mod fwf;
pub mod json;
pub mod jsonl;
pub mod logfmt;
pub mod parquet;
pub mod sqlite;
pub mod tsv;

fn final_step(source: Source, rtdf: impl ReadToDataFrames) {
    Message::AppDismissOverlay.enqueue();
    match rtdf.named_frames(source.clone()) {
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
