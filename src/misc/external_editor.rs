use std::{env, process::Command};

use anyhow::anyhow;
use polars::frame::DataFrame;

use crate::{
    AppResult,
    handler::event::lock_event,
    reader::{CsvToDataFrame, ReadToDataFrames, Source},
    tui::terminal::{invalidate_tui, start_tui, stop_tui},
    writer::{Destination, WriteToCsv, WriteToFile},
};

#[derive(Debug)]
pub struct ExternalEditor {
    df: DataFrame,
}

impl ExternalEditor {
    pub fn new(df: DataFrame) -> Self {
        Self { df }
    }

    pub fn edit(mut self) -> AppResult<DataFrame> {
        let editor = env::var("EDITOR").map_err(|_| anyhow!("$EDITOR is not set"))?;
        let tempfile = tempfile::NamedTempFile::new()?;

        WriteToCsv::default()
            .with_header(true)
            .with_quote_char('"')
            .with_separator_char(',')
            .write_to_file(Destination::File(tempfile.path().to_owned()), &mut self.df)?;

        let editor_status = {
            let _lock = lock_event();
            // terminal::disable_raw_mode()?;
            // crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
            stop_tui()?;
            let status = Command::new(editor).arg(tempfile.path()).status();
            // terminal::enable_raw_mode().unwrap_or_graceful_shutdown();
            // crossterm::execute!(io::stdout(), EnterAlternateScreen).unwrap_or_graceful_shutdown();
            start_tui()?;
            invalidate_tui();
            status
        }?;

        if editor_status.success() {
            let df = CsvToDataFrame::default()
                .with_no_header(false)
                .with_quote_char('"')
                .with_separator(',')
                .named_frames(Source::File(tempfile.path().to_owned()))?
                .into_iter()
                .next()
                .map(|(_, df)| df)
                .ok_or(anyhow!("Failed to load data frame back from the editor"))?;
            Ok(df)
        } else {
            Err(anyhow!("Editor failed"))
        }
    }
}
