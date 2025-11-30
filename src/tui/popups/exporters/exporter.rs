use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    handler::action::Action,
    misc::type_ext::UnwrapOrEnqueueError,
    tui::component::Component,
    writer::{
        Destination, JsonFormat, WriteToArrow, WriteToCsv, WriteToFile, WriteToJson, WriteToParquet,
    },
};

#[derive(Debug)]
pub enum Export {
    WaitingForUserInput,
    ArrowToFile(PathBuf),
    CsvToFile(char, char, bool, PathBuf),
    CsvToClipboard(char, char, bool),
    JsonToFile(PathBuf),
    JsonToClipboard,
    JsonLToFile(PathBuf),
    JsonLToClipboard,
    ParquetToFile(PathBuf),
    TsvToFile(PathBuf),
    TsvToClipboard,
}

pub trait State: Default {
    fn next(self) -> Self;
    fn responder(&mut self) -> Option<&mut dyn Component>;
    fn export(&self) -> Export;
}
#[derive(Debug)]
pub struct Exporter<S> {
    state: S,
    df: DataFrame,
}

impl<S> Exporter<S>
where
    S: State,
{
    pub fn new(df: DataFrame) -> Self {
        Self {
            state: Default::default(),
            df,
        }
    }
}

impl<S> Component for Exporter<S>
where
    S: State,
{
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        if let Some(responder) = self.state.responder() {
            responder.render(area, buf, focus_state);
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if let Some(responder) = self.state.responder() {
            responder.handle(event)
                || match (event.code, event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        Action::PaneDismissModal.enqueue();
                        true
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        self.state = std::mem::take(&mut self.state).next();

                        match self.state.export() {
                            Export::WaitingForUserInput => (),
                            Export::ArrowToFile(path_buf) => {
                                WriteToArrow
                                    .write_to_file(Destination::File(path_buf), &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::CsvToFile(sep, quote, header, path_buf) => {
                                WriteToCsv::default()
                                    .with_separator_char(sep)
                                    .with_quote_char(quote)
                                    .with_header(header)
                                    .write_to_file(Destination::File(path_buf), &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::CsvToClipboard(sep, quote, header) => {
                                WriteToCsv::default()
                                    .with_separator_char(sep)
                                    .with_quote_char(quote)
                                    .with_header(header)
                                    .write_to_file(Destination::Clipboard, &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::JsonToFile(path_buf) => {
                                WriteToJson::default()
                                    .with_format(JsonFormat::Json)
                                    .write_to_file(Destination::File(path_buf), &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::JsonToClipboard => {
                                WriteToJson::default()
                                    .with_format(JsonFormat::Json)
                                    .write_to_file(Destination::Clipboard, &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::JsonLToFile(path_buf) => {
                                WriteToJson::default()
                                    .with_format(JsonFormat::JsonLine)
                                    .write_to_file(Destination::File(path_buf), &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::JsonLToClipboard => {
                                WriteToJson::default()
                                    .with_format(JsonFormat::JsonLine)
                                    .write_to_file(Destination::Clipboard, &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::ParquetToFile(path_buf) => {
                                WriteToParquet
                                    .write_to_file(Destination::File(path_buf), &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::TsvToFile(path_buf) => {
                                WriteToCsv::default()
                                    .with_separator_char('\t')
                                    .with_quote_char('"')
                                    .with_header(false)
                                    .write_to_file(Destination::File(path_buf), &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                            Export::TsvToClipboard => {
                                WriteToCsv::default()
                                    .with_separator_char('\t')
                                    .with_quote_char('"')
                                    .with_header(false)
                                    .write_to_file(Destination::Clipboard, &mut self.df)
                                    .unwrap_or_enqueue_error();
                            }
                        }
                        true
                    }
                    _ => false,
                }
        } else {
            false
        }
    }
}
