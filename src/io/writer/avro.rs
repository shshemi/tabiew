use std::fs::File;

use polars::{
    frame::DataFrame,
    io::{SerWriter, avro::AvroWriter},
};

use crate::{AppResult, misc::osc52::CopyToClipboardOsc52};

use super::traits::{Destination, WriteToFile};

#[derive(Debug, Default)]
pub struct WriteToAvro;

impl WriteToFile for WriteToAvro {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => Ok(AvroWriter::new(File::create(path)?).finish(data_frame)?),
            Destination::Clipboard => {
                let mut buf = Vec::new();
                AvroWriter::new(&mut buf).finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}
