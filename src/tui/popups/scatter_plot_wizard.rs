use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    tui::{
        pickers::search_picker::SearchPicker,
        popups::wizard::{Wizard, WizardState},
    },
};

pub type ScatterPlotWizard = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickX {
        df: DataFrame,
        picker: SearchPicker<String>,
    },
    PickY {
        df: DataFrame,
        x: String,
        picker: SearchPicker<String>,
    },
    PickColorBy {
        x: String,
        y: String,
        picker: SearchPicker<String>,
    },
}

impl State {
    pub fn new(df: DataFrame) -> Self {
        let items = df
            .column_iter()
            .filter(|col| col.dtype().is_numeric())
            .map(|col| col.name().to_string())
            .collect();
        Self::PickX {
            df,
            picker: SearchPicker::new(items).with_title("Axis X"),
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickX { df, picker } => {
                if let Some(x) = picker.selected_item().cloned() {
                    let items = picker
                        .into_items()
                        .into_iter()
                        .filter(|item| item != &x)
                        .collect();
                    State::PickY {
                        df,
                        x,
                        picker: SearchPicker::new(items).with_title("Axis Y"),
                    }
                } else {
                    State::PickX { df, picker }
                }
            }
            State::PickY { df, x, picker } => {
                if let Some(y) = picker.selected_item().cloned() {
                    State::PickColorBy {
                        x,
                        y,
                        picker: SearchPicker::new(
                            std::iter::once("None".to_owned())
                                .chain(
                                    df.column_iter()
                                        .filter(|col| {
                                            let dtype = col.dtype();
                                            dtype.is_string()
                                                || dtype.is_bool()
                                                || dtype.is_integer()
                                        })
                                        .map(|col| col.name().to_string()),
                                )
                                .collect(),
                        )
                        .with_title("Color By"),
                    }
                } else {
                    State::PickY { df, x, picker }
                }
            }
            State::PickColorBy { x, y, picker } => {
                if let Some(grp) = picker.selected_item().cloned() {
                    let x = x.clone();
                    let y = y.clone();
                    if picker.selected() == Some(0) {
                        Message::PaneShowScatterPlot(x, y, None).enqueue();
                    } else {
                        Message::PaneShowScatterPlot(x, y, Some(grp)).enqueue();
                    }
                }
                State::PickColorBy { x, y, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickX { df: _, picker } => picker,
            State::PickY {
                df: _,
                x: _,
                picker,
            } => picker,
            State::PickColorBy { x: _, y: _, picker } => picker,
        }
    }
}
