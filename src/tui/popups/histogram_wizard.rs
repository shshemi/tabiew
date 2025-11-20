use crossterm::event::KeyEvent;
use polars::{
    frame::DataFrame,
    prelude::{Column, DataType, PlSmallStr},
};
use ratatui::widgets::StatefulWidget;

use crate::tui::{
    pickers::{
        search_picker::{SearchPicker, SearchPicker},
        text_picker::{TextPicker, TextPicker},
    },
    widgets::input::InputType,
};

#[derive(Debug)]
pub enum HistogramWizardState {
    ColumnSelect {
        columns: Vec<String>,
        picker: SearchPicker,
    },
    BucketCount {
        column: String,
        picker: TextPicker,
    },
    Show {
        column: String,
        buckets: usize,
    },
}

impl HistogramWizardState {
    pub fn new(df: &DataFrame) -> Self {
        HistogramWizardState::ColumnSelect {
            columns: df
                .column_iter()
                .filter(|col| {
                    matches!(
                        col.dtype(),
                        DataType::UInt8
                            | DataType::UInt16
                            | DataType::UInt32
                            | DataType::UInt64
                            | DataType::Int8
                            | DataType::Int16
                            | DataType::Int32
                            | DataType::Int64
                            | DataType::Int128
                            | DataType::Float32
                            | DataType::Float64
                            | DataType::Decimal(_, _)
                            | DataType::Boolean
                            | DataType::String
                    )
                })
                .map(Column::name)
                .map(PlSmallStr::to_string)
                .collect(),
            picker: Default::default(),
        }
    }

    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            HistogramWizardState::ColumnSelect { columns, picker } => {
                let column = picker
                    .selected()
                    .and_then(|i| columns.get(i))
                    .map(|m| m.to_owned())
                    .unwrap_or("Default".to_owned());
                HistogramWizardState::BucketCount {
                    column,
                    picker: TextPicker::default()
                        .with_value("38".to_owned())
                        .with_input_type(InputType::Numeric),
                }
            }
            HistogramWizardState::BucketCount { column, picker } => {
                let buckets = picker.value().parse().unwrap_or(1);
                HistogramWizardState::Show { column, buckets }
            }
            HistogramWizardState::Show { column, buckets } => {
                HistogramWizardState::Show { column, buckets }
            }
        }
    }

    pub fn select_next(&mut self) {
        match self {
            HistogramWizardState::ColumnSelect { columns: _, picker } => {
                picker.list_mut().select_next();
            }
            HistogramWizardState::BucketCount {
                column: _,
                picker: _,
            } => (),
            HistogramWizardState::Show {
                column: _,
                buckets: _,
            } => (),
        }
    }

    pub fn select_previous(&mut self) {
        match self {
            HistogramWizardState::ColumnSelect { columns: _, picker } => {
                picker.list_mut().select_previous()
            }
            HistogramWizardState::BucketCount {
                column: _,
                picker: _,
            } => (),
            HistogramWizardState::Show {
                column: _,
                buckets: _,
            } => (),
        }
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match self {
            HistogramWizardState::ColumnSelect { columns: _, picker } => {
                picker.input_mut().handle(event);
            }
            HistogramWizardState::BucketCount { column: _, picker } => {
                picker.input_mut().handle(event);
            }
            HistogramWizardState::Show {
                column: _,
                buckets: _,
            } => (),
        }
    }
}

impl Default for HistogramWizardState {
    fn default() -> Self {
        HistogramWizardState::ColumnSelect {
            columns: Default::default(),
            picker: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct HistogramWizard {}

impl StatefulWidget for HistogramWizard {
    type State = HistogramWizardState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            HistogramWizardState::ColumnSelect { columns, picker } => {
                SearchPicker::default()
                    .items(columns.iter().map(String::as_str))
                    .title("Column")
                    .render(area, buf, picker);
            }
            HistogramWizardState::BucketCount { column: _, picker } => {
                TextPicker::default()
                    .title("Suggested Bucket Count")
                    .render(area, buf, picker);
            }
            HistogramWizardState::Show {
                column: _,
                buckets: _,
            } => (),
        }
    }
}
