use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::{
    handler::message::Message,
    tui::{
        icons,
        popups::{
            pickers::search_picker::SearchPicker,
            wizard::{Wizard, WizardStep},
        },
    },
};

pub type ColumnCaster = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickColumn {
        columns: Vec<(String, String)>,
        picker: SearchPicker<String>,
    },
    PickType {
        columns: Vec<(String, String)>,
        col_name: String,
        picker: SearchPicker<TargetType>,
    },
}

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickColumn { columns, picker } => {
                if let Some(col_name) = picker.selected_str() {
                    Self::PickType {
                        columns,
                        col_name: col_name.to_owned(),
                        picker: SearchPicker::new(TargetType::iter().collect())
                            .with_title(icons::CAST.title("Type")),
                    }
                } else {
                    Self::PickColumn { columns, picker }
                }
            }
            State::PickType {
                columns,
                col_name,
                picker,
            } => {
                if let Some(target_type) = picker.selected_item() {
                    Message::PaneDismissModal.enqueue();
                    let current = columns
                        .iter()
                        .find(|(name, _)| *name == col_name)
                        .map(|(_, data_type)| data_type.as_str());
                    match current.and_then(TargetType::from_duckdb) == Some(*target_type) {
                        true => Message::AppShowError(format!(
                            "Column '{col_name}' is already {target_type}"
                        ))
                        .enqueue(),
                        false => Message::AppCast(col_name.clone(), *target_type).enqueue(),
                    }
                }
                State::PickType {
                    columns,
                    col_name,
                    picker,
                }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickColumn { columns: _, picker } => picker,
            State::PickType {
                columns: _,
                col_name: _,
                picker,
            } => picker,
        }
    }
}

impl From<Vec<(String, String)>> for State {
    fn from(columns: Vec<(String, String)>) -> Self {
        State::PickColumn {
            picker: SearchPicker::new(columns.iter().map(|(name, _)| name.clone()).collect())
                .with_title(icons::COLUMN.title("Column")),
            columns,
        }
    }
}

impl TargetType {
    pub fn duckdb(&self) -> &'static str {
        match self {
            TargetType::Boolean => "BOOLEAN",
            TargetType::Date => "DATE",
            TargetType::Datetime => "TIMESTAMP",
            TargetType::Float => "DOUBLE",
            TargetType::Int => "BIGINT",
            TargetType::String => "VARCHAR",
        }
    }
    fn from_duckdb(data_type: &str) -> Option<Self> {
        TargetType::iter().find(|target| target.duckdb() == data_type)
    }
}

#[derive(Debug, Clone, PartialEq, Copy, IntoStaticStr, EnumIter, Display)]
pub enum TargetType {
    Boolean,
    Date,
    Datetime,
    Float,
    Int,
    String,
}
