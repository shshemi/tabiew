use crossterm::event::KeyEvent;
use polars::{
    frame::DataFrame,
    prelude::{Column, PlSmallStr},
};

use crate::tui::{
    component::Component,
    pickers::{search_picker::SearchPicker, text_picker::TextPicker},
    widgets::input::InputType,
};

#[derive(Debug)]
pub enum HistogramWizard {
    ColumnSelect { picker: SearchPicker<String> },
    BucketCount { column: String, picker: TextPicker },
    Show { column: String, buckets: usize },
}

impl HistogramWizard {
    pub fn new(df: &DataFrame) -> Self {
        HistogramWizard::ColumnSelect {
            picker: SearchPicker::new(
                df.column_iter()
                    .filter(|col| {
                        let dtype = col.dtype();
                        dtype.is_numeric() || dtype.is_string() || dtype.is_bool()
                    })
                    .map(Column::name)
                    .map(PlSmallStr::to_string)
                    .collect(),
            ),
        }
    }

    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            HistogramWizard::ColumnSelect { picker } => {
                if let Some(column) = picker.selected_str() {
                    HistogramWizard::BucketCount {
                        column: column.to_owned(),
                        picker: TextPicker::default()
                            .with_value("38".to_owned())
                            .with_input_type(InputType::Numeric),
                    }
                } else {
                    HistogramWizard::ColumnSelect { picker }
                }
            }
            HistogramWizard::BucketCount { column, picker } => {
                let buckets = picker.value().parse().unwrap_or(1);
                HistogramWizard::Show { column, buckets }
            }
            HistogramWizard::Show { column, buckets } => HistogramWizard::Show { column, buckets },
        }
    }
}

impl Component for HistogramWizard {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        match self {
            HistogramWizard::ColumnSelect { picker } => {
                picker.render(area, buf, focus_state);
            }
            HistogramWizard::BucketCount { column: _, picker } => {
                picker.render(area, buf, focus_state);
            }
            HistogramWizard::Show {
                column: _,
                buckets: _,
            } => (),
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        match self {
            HistogramWizard::ColumnSelect { picker } => picker.handle(event),
            HistogramWizard::BucketCount { column: _, picker } => picker.handle(event),
            HistogramWizard::Show {
                column: _,
                buckets: _,
            } => false,
        }
    }
}

impl Default for HistogramWizard {
    fn default() -> Self {
        HistogramWizard::ColumnSelect {
            picker: SearchPicker::new(Vec::default()),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct HistogramWizard {}

// impl StatefulWidget for HistogramWizard {
//     type State = HistogramWizardState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             HistogramWizardState::ColumnSelect { columns, picker } => {
//                 SearchPicker::default()
//                     .items(columns.iter().map(String::as_str))
//                     .title("Column")
//                     .render(area, buf, picker);
//             }
//             HistogramWizardState::BucketCount { column: _, picker } => {
//                 TextPicker::default()
//                     .title("Suggested Bucket Count")
//                     .render(area, buf, picker);
//             }
//             HistogramWizardState::Show {
//                 column: _,
//                 buckets: _,
//             } => (),
//         }
//     }
// }
