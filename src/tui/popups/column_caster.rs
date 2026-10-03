use anyhow::bail;
use polars::{
    frame::DataFrame,
    prelude::{DataType, TimeUnit},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::{
    AppResult,
    handler::message::Message,
    parsers::auto_parser::AutoParser,
    tui::{
        icons,
        pane::TableDescription,
        popups::{
            search_picker::SearchPicker,
            wizard::{Wizard, WizardStep},
        },
    },
};

pub type ColumnCaster = Wizard<State>;

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

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickColumn { df, picker } => {
                if let Some(col_name) = picker.selected_str() {
                    Self::PickType {
                        df,
                        col_name: col_name.to_owned(),
                        picker: SearchPicker::new(TargetType::iter().collect())
                            .with_title(icons::CAST.title("Type")),
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
                        Ok(_) => {
                            Message::PanePushDataFrame(
                                df.clone(),
                                TableDescription::Cast(format!("'{col_name}' as {target_type}")),
                            )
                            .enqueue();
                            Message::AppShowToast(format!(
                                "Column '{}' were casted to '{}'",
                                col_name, target_type
                            ))
                            .enqueue();
                        }
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
                    .columns()
                    .iter()
                    .map(|col| col.name().as_str().to_owned())
                    .collect(),
            )
            .with_title(icons::COLUMN.title("Column")),
            df: value,
        }
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

impl From<TargetType> for DataType {
    fn from(value: TargetType) -> Self {
        match value {
            TargetType::Boolean => DataType::Boolean,
            TargetType::Date => DataType::Date,
            TargetType::Datetime => DataType::Datetime(TimeUnit::Milliseconds, None),
            TargetType::Float => DataType::Float64,
            TargetType::Int => DataType::Int64,
            TargetType::String => DataType::String,
        }
    }
}

impl TryFrom<&DataType> for TargetType {
    type Error = ();

    fn try_from(dtype: &DataType) -> Result<Self, Self::Error> {
        match dtype {
            DataType::Boolean => Ok(TargetType::Boolean),
            DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::UInt128
            | DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::Int128 => Ok(TargetType::Int),
            DataType::Float16 | DataType::Float32 | DataType::Float64 => Ok(TargetType::Float),
            DataType::String => Ok(TargetType::String),
            DataType::Date => Ok(TargetType::Date),
            DataType::Datetime(_, _) => Ok(TargetType::Datetime),
            _ => Err(()),
        }
    }
}

fn cast_column(df: &mut DataFrame, name: &str, target_type: TargetType) -> AppResult<()> {
    let column = df.column(name)?;
    if TargetType::try_from(column.dtype()) == Ok(target_type) {
        bail!("Column '{}' is already {}", name, target_type)
    }
    let casted = if column.dtype().is_string() {
        match target_type {
            TargetType::Boolean => AutoParser::default().with_bool(true).parse_strict(column)?,
            TargetType::Date => AutoParser::default().with_date(true).parse_strict(column)?,
            TargetType::Datetime => AutoParser::default()
                .with_datetime(true)
                .parse_strict(column)?,
            TargetType::Float => AutoParser::default()
                .with_float(true)
                .parse_strict(column)?,
            TargetType::Int => AutoParser::default().with_int(true).parse_strict(column)?,
            TargetType::String => unreachable!(),
        }
    } else {
        let casted = column.cast(&target_type.into())?;
        if casted.null_count() == column.null_count() {
            casted
        } else {
            bail!("Column '{}' cannot be refined to {}", name, target_type)
        }
    };
    df.replace(name, casted)?;
    Ok(())
}
