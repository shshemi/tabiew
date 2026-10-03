use polars::frame::DataFrame;

use crate::{AppResult, readers::ReaderSource};

pub type NamedFrame = (String, DataFrame);
pub type NamedFrames = Box<[NamedFrame]>;

pub trait DataFrameReader {
    fn read_to_data_frames(&self, source: ReaderSource) -> AppResult<NamedFrames>;
}
