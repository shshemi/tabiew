use crate::{
    handler::message::Message,
    misc::globals::sql,
    reader::{ReadToDataFrames, Source},
};

pub mod arrow_importer;
pub mod csv_importer;
pub mod excel_importer;
pub mod fwf_importer;
pub mod json_importer;
pub mod jsonl_importer;
pub mod parquet_importer;
pub mod sqlite_importer;
pub mod tsv_importer;

fn final_step(source: Source, rtdf: impl ReadToDataFrames) {
    Message::AppDismissOverlay.enqueue();
    match rtdf.named_frames(source.clone()) {
        Ok(named_frames) => {
            for (name, df) in named_frames {
                let name = sql().register(&name, df.clone(), source.clone());
                Message::TabsAddNamePane(df, name).enqueue();
            }
        }
        Err(err) => Message::AppShowError(err.to_string()).enqueue(),
    }
}
