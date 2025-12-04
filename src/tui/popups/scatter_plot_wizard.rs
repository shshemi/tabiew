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
        picker: SearchPicker<String>,
        group_by_item: Vec<String>,
    },
    PickY {
        x: String,
        picker: SearchPicker<String>,
        group_by_item: Vec<String>,
    },
    PickGroupBy {
        x: String,
        y: String,
        picker: SearchPicker<String>,
    },
    Visualize {
        x: String,
        y: String,
        group_by: Option<String>,
    },
}

impl State {
    pub fn new(df: DataFrame) -> Self {
        let items = df
            .column_iter()
            .filter(|col| col.dtype().is_numeric())
            .map(|col| col.name().to_string())
            .collect();
        let group_by_items = std::iter::once("None".to_owned())
            .chain(
                df.column_iter()
                    .filter(|col| {
                        let dtype = col.dtype();
                        dtype.is_string() || dtype.is_bool() || dtype.is_integer()
                    })
                    .map(|col| col.name().to_string()),
            )
            .collect();
        Self::PickX {
            picker: SearchPicker::new(items).with_title("Axis X"),
            group_by_item: group_by_items,
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickX {
                picker,
                group_by_item,
            } => {
                if let Some(x) = picker.selected_item().cloned() {
                    let items = picker.into_items();
                    State::PickY {
                        x,
                        picker: SearchPicker::new(items).with_title("Axis Y"),
                        group_by_item,
                    }
                } else {
                    State::PickX {
                        picker,
                        group_by_item,
                    }
                }
            }
            State::PickY {
                x,
                picker,
                group_by_item,
            } => {
                if let Some(y) = picker.selected_item().cloned() {
                    State::PickGroupBy {
                        x,
                        y,
                        picker: SearchPicker::new(group_by_item).with_title("Group By"),
                    }
                } else {
                    State::PickY {
                        x,
                        picker,
                        group_by_item,
                    }
                }
            }
            State::PickGroupBy { x, y, picker } => {
                if let Some(grp) = picker.selected_item().cloned() {
                    let x = x.clone();
                    let y = y.clone();
                    if picker.selected() == Some(0) {
                        Message::PaneDismissModal.enqueue();
                        Message::PaneShowScatterPlot(x, y, None).enqueue();
                    } else {
                        Message::PaneDismissModal.enqueue();
                        Message::PaneShowScatterPlot(x, y, Some(grp)).enqueue();
                    }
                }
                State::PickGroupBy { x, y, picker }
            }
            State::Visualize {
                x: _,
                y: _,
                group_by: _,
            } => self,
        }
    }

    fn responder(&mut self) -> Option<&mut dyn crate::tui::component::Component> {
        match self {
            State::PickX {
                picker,
                group_by_item: _,
            } => Some(picker),
            State::PickY {
                x: _x,
                picker,
                group_by_item: _,
            } => Some(picker),
            State::PickGroupBy {
                x: _x,
                y: _y,
                picker,
            } => Some(picker),
            State::Visualize {
                x: _x,
                y: _y,
                group_by: _group_by,
            } => None,
        }
    }

    fn finalize(&self) -> bool {
        if let State::Visualize { x, y, group_by } = self {
            Message::PaneShowScatterPlot(x.to_owned(), y.to_owned(), group_by.to_owned()).enqueue();
            true
        } else {
            false
        }
    }
}
