use anyhow::anyhow;
use polars::{
    frame::DataFrame,
    prelude::{DataType, TimeUnit},
    series::ChunkCompareEq,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::{
    AppResult,
    handler::message::Message,
    tui::{
        pane::TableDescription,
        pickers::search_picker::SearchPicker,
        popups::wizard::{Wizard, WizardState},
    },
};

pub type ColumnCastWizard = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickColumn {
        df: DataFrame,
        picker: SearchPicker<String>,
    },
    PickType {
        df: DataFrame,
        col_name: String,
        picker: SearchPicker<TargetType>,
    },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickColumn { df, picker } => {
                if let Some(col_name) = picker.selected_str() {
                    Self::PickType {
                        df,
                        col_name: col_name.to_owned(),
                        picker: SearchPicker::new(TargetType::iter().collect()).with_title("Type"),
                    }
                } else {
                    Self::PickColumn { df, picker }
                }
            }
            State::PickType {
                mut df,
                col_name,
                picker,
            } => {
                if let Some(target_type) = picker.selected_item() {
                    Message::PaneDismissModal.enqueue();
                    match cast_column(&mut df, &col_name, *target_type) {
                        Ok(_) => Message::PanePushDataFrame(
                            df.clone(),
                            TableDescription::Cast(format!("{col_name}: {target_type}")),
                        )
                        .enqueue(),
                        Err(err) => Message::AppShowError(err.to_string()).enqueue(),
                    }
                }
                State::PickType {
                    df,
                    col_name,
                    picker,
                }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickColumn { df: _, picker } => picker,
            State::PickType {
                df: _,
                col_name: _,
                picker,
            } => picker,
        }
    }
}

impl From<DataFrame> for State {
    fn from(value: DataFrame) -> Self {
        State::PickColumn {
            picker: SearchPicker::new(
                value
                    .column_iter()
                    .map(|col| col.name().as_str().to_owned())
                    .collect(),
            )
            .with_title("Column"),
            df: value,
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter, Display)]
pub enum TargetType {
    Date,
    Datetime,
    Float,
    Int,
    String,
}

impl From<TargetType> for DataType {
    fn from(value: TargetType) -> Self {
        match value {
            TargetType::Date => DataType::Date,
            TargetType::Datetime => DataType::Datetime(TimeUnit::Milliseconds, None),
            TargetType::Float => DataType::Float64,
            TargetType::Int => DataType::Int64,
            TargetType::String => DataType::String,
        }
    }
}

fn cast_column(df: &mut DataFrame, name: &str, target_type: TargetType) -> AppResult<()> {
    let col = df.column(name)?;
    let new_col = col.cast(&target_type.into())?;
    if col.is_null().equal(&new_col.is_null()).all() {
        df.with_column(new_col)?;
        Ok(())
    } else {
        Err(anyhow!("Column '{name}' cannot be casted to {target_type}"))
    }
}
