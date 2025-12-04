use polars::frame::DataFrame;

use crate::tui::{
    pickers::search_picker::SearchPicker,
    popups::wizard::{Wizard, WizardState},
};

pub type ScatterPlotWizard = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickX {
        picker: SearchPicker<String>,
    },
    PickY {
        x: String,
        picker: SearchPicker<String>,
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
        Self::PickX {
            picker: SearchPicker::new(items),
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickX { picker } => {
                if let Some(x) = picker.selected_item().cloned() {
                    let items = picker.into_items();
                    State::PickY {
                        x,
                        picker: SearchPicker::new(items),
                    }
                } else {
                    State::PickX { picker }
                }
            }
            State::PickY { x, picker } => {
                if let Some(y) = picker.selected_item().cloned() {
                    let items = std::iter::once("None".to_owned())
                        .chain(picker.into_items())
                        .collect();
                    State::PickGroupBy {
                        x,
                        y,
                        picker: SearchPicker::new(items),
                    }
                } else {
                    //
                    State::PickY { x, picker }
                }
            }
            State::PickGroupBy { x, y, picker } => {
                if let Some(grp) = picker.selected_item().cloned() {
                    if picker.selected() == Some(0) {
                        //
                        State::Visualize {
                            x,
                            y,
                            group_by: None,
                        }
                    } else {
                        State::Visualize {
                            x,
                            y,
                            group_by: Some(grp),
                        }
                    }
                } else {
                    //
                    State::PickGroupBy { x, y, picker }
                }
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
            State::PickX { picker } => Some(picker),
            State::PickY { x: _x, picker } => Some(picker),
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
        matches!(
            self,
            State::Visualize {
                x: _,
                y: _,
                group_by: _
            }
        )
    }
}
