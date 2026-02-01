use std::{env, io, process::Command, sync::atomic::Ordering};

use anyhow::anyhow;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use polars::frame::DataFrame;

use crate::{
    AppResult,
    handler::event::EVENT_MUTEX,
    misc::type_ext::UnwrapOrGracefulShutdown,
    reader::{CsvToDataFrame, ReadToDataFrames, Source},
    tui::terminal::INVALIDATE_DRAW,
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
            let _lock = EVENT_MUTEX.lock().unwrap_or_graceful_shutdown();
            terminal::disable_raw_mode()?;
            crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
            let status = Command::new(editor).arg(tempfile.path()).status();
            terminal::enable_raw_mode().unwrap_or_graceful_shutdown();
            crossterm::execute!(io::stdout(), EnterAlternateScreen).unwrap_or_graceful_shutdown();
            INVALIDATE_DRAW.store(true, Ordering::Relaxed);
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
