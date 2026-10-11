use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    pub height: usize,
    pub width: usize,
}

impl From<&DataFrame> for Shape {
    fn from(value: &DataFrame) -> Self {
        Self {
            height: value.height(),
            width: value.width(),
        }
    }
}
