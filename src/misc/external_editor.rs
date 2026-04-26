use std::{env, process::Command};

use anyhow::anyhow;
use polars::frame::DataFrame;

use crate::{
    AppResult,
    handler::event::{disable_event_read, enable_event_read},
    io::reader::{CsvToDataFrame, ReadToDataFrames, Source},
    io::writer::{Destination, WriteToCsv, WriteToFile},
    misc::type_inferer::TypeInferer,
    tui::terminal::{invalidate_tui, start_tui, stop_tui},
};

pub fn edit_in_external_editor(mut df: DataFrame) -> AppResult<DataFrame> {
    let editor = env::var("EDITOR").map_err(|_| anyhow!("$EDITOR is not set"))?;
    let tempfile = tempfile::NamedTempFile::new()?;

    WriteToCsv::default()
        .with_header(true)
        .with_quote_char('"')
        .with_separator_char(',')
        .write_to_file(Destination::File(tempfile.path().to_owned()), &mut df)?;

    let editor_status = {
        disable_event_read();
        stop_tui()?;
        let status = Command::new(editor).arg(tempfile.path()).status();
        start_tui()?;
        invalidate_tui();
        enable_event_read();
        status
    }?;

    if editor_status.success() {
        let mut df = CsvToDataFrame::default()
            .with_no_header(false)
            .with_quote_char('"')
            .with_separator(',')
            .read_to_data_frames(Source::File(tempfile.path().to_owned()))?
            .into_iter()
            .next()
            .map(|(_, df)| df)
            .ok_or(anyhow!("Failed to load data frame back from the editor"))?;
        let ti = TypeInferer::default().boolean().int().float();
        ti.update(&mut df);
        Ok(df)
    } else {
        Err(anyhow!("Editor failed"))
    }
}
