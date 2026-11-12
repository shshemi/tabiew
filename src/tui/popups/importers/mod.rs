use crate::{
    duckdb::{DataSource, Import},
    handler::message::Message,
};

pub mod arrow;
pub mod avro;
pub mod csv;
pub mod excel;
pub mod fwf;
pub mod html;
pub mod json;
pub mod jsonl;
pub mod logfmt;
pub mod markdown;
pub mod parquet;
pub mod sqlite;
pub mod tsv;

fn dismiss_overlay_and_load_data_frame(source: DataSource, import: Import) {
    Message::AppDismissOverlay.enqueue();
    Message::AppImport(source, import).enqueue();
}
