use std::fs::File;

use polars::{frame::DataFrame, io::SerWriter, prelude::JsonWriter};

use crate::{AppResult, misc::osc52::CopyToClipboardOsc52};

use super::traits::{Destination, WriteToFile};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonFormat {
    #[default]
    Json,
    JsonLine,
}

impl From<JsonFormat> for polars::prelude::JsonFormat {
    fn from(value: JsonFormat) -> Self {
        match value {
            JsonFormat::Json => polars::prelude::JsonFormat::Json,
            JsonFormat::JsonLine => polars::prelude::JsonFormat::JsonLines,
        }
    }
}

#[derive(Debug, Default)]
pub struct WriteToJson {
    fmt: JsonFormat,
}

impl WriteToJson {
    pub fn with_format(mut self, fmt: JsonFormat) -> Self {
        self.fmt = fmt;
        self
    }
}

impl WriteToFile for WriteToJson {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => Ok(JsonWriter::new(File::create(path)?)
                .with_json_format(self.fmt.into())
                .finish(data_frame)?),
            Destination::Clipboard => {
                let mut buf = Vec::new();
                JsonWriter::new(&mut buf)
                    .with_json_format(self.fmt.into())
                    .finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}
