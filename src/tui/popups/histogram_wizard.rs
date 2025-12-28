use std::fmt::Display;

use polars::{frame::DataFrame, prelude::DataType};

use crate::{
    handler::message::Message,
    tui::{
        pickers::{search_picker::SearchPicker, text_picker::TextPicker},
        popups::wizard::{Wizard, WizardState},
        widgets::input::InputType,
    },
};

const DEFAULT_BUCKET_COUNT: &str = "24";

pub type HistogramWizard = Wizard<State>;

#[derive(Debug)]
pub enum State {
    ColumnSelect {
        picker: SearchPicker<ColumnNameType>,
    },
    BucketCount {
        column: String,
        picker: TextPicker,
    },
}

impl State {
    pub fn new(df: &DataFrame) -> Self {
        let items = df
            .column_iter()
            .filter(|col| {
                let dtype = col.dtype();
                dtype.is_numeric() || dtype.is_string() || dtype.is_bool()
            })
            .map(|col| ColumnNameType(col.name().to_string(), col.dtype().to_owned()))
            .collect();

        State::ColumnSelect {
            picker: SearchPicker::new(items),
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::ColumnSelect { picker } => {
                if let Some(ColumnNameType(name, dtype)) = picker.selected_item() {
                    if dtype.is_string() {
                        Message::PaneShowHistogram(name.clone(), 0).enqueue();

                        State::ColumnSelect { picker }
                    } else {
                        State::BucketCount {
                            column: name.to_owned(),
                            picker: TextPicker::default()
                                .with_input_type(InputType::Numeric)
                                .with_value(DEFAULT_BUCKET_COUNT.to_owned()),
                        }
                    }
                } else {
                    State::ColumnSelect { picker }
                }
            }
            State::BucketCount { column, picker } => {
                let buckets = picker.value().parse().unwrap_or(1);
                Message::PaneShowHistogram(column.clone(), buckets).enqueue();
                State::BucketCount { column, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::ColumnSelect { picker } => picker,
            State::BucketCount { column: _, picker } => picker,
        }
    }
}

#[derive(Debug)]
pub struct ColumnNameType(String, DataType);

impl Display for ColumnNameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
