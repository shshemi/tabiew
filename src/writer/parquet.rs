use std::fs::File;

use polars::{frame::DataFrame, prelude::ParquetWriter};

use crate::{AppResult, misc::osc52::CopyToClipboardOsc52};

use super::traits::{Destination, WriteToFile};

#[derive(Debug, Default)]
pub struct WriteToParquet;

impl WriteToFile for WriteToParquet {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => {
                ParquetWriter::new(File::create(path)?).finish(data_frame)?;
                Ok(())
            }
            Destination::Clipboard => {
                let mut buf = Vec::new();
                ParquetWriter::new(&mut buf).finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}
