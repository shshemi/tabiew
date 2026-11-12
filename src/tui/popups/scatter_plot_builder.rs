use crate::{
    handler::message::Message,
    reader::{is_integer, is_numeric},
    tui::{
        icons,
        popups::{
            pickers::search_picker::SearchPicker,
            wizard::{Wizard, WizardStep},
        },
    },
};

pub type ScatterPlotBuilder = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickX {
        columns: Vec<(String, String)>,
        picker: SearchPicker<String>,
    },
    PickY {
        columns: Vec<(String, String)>,
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
    pub fn new(columns: Vec<(String, String)>) -> Self {
        let items = columns
            .iter()
            .filter(|(_, dtype)| is_numeric(dtype))
            .map(|(name, _)| name.clone())
            .collect();
        Self::PickX {
            columns,
            picker: SearchPicker::new(items).with_title(icons::WIDTH.title("Axis X")),
        }
    }
}

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickX { columns, picker } => {
                if let Some(x) = picker.selected_item().cloned() {
                    let items = picker
                        .into_items()
                        .into_iter()
                        .filter(|item| item != &x)
                        .collect();
                    State::PickY {
                        columns,
                        x,
                        picker: SearchPicker::new(items).with_title(icons::HEIGHT.title("Axis Y")),
                    }
                } else {
                    State::PickX { columns, picker }
                }
            }
            State::PickY { columns, x, picker } => {
                if let Some(y) = picker.selected_item().cloned() {
                    State::PickColorBy {
                        x,
                        y,
                        picker: SearchPicker::new(
                            std::iter::once("None".to_owned())
                                .chain(
                                    columns
                                        .iter()
                                        .filter(|(_, dtype)| {
                                            dtype == "VARCHAR"
                                                || dtype == "BOOLEAN"
                                                || is_integer(dtype)
                                        })
                                        .map(|(name, _)| name.clone()),
                                )
                                .collect(),
                        )
                        .with_title(icons::PALETTE.title("Color By")),
                    }
                } else {
                    State::PickY { columns, x, picker }
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
            State::PickX { columns: _, picker } => picker,
            State::PickY {
                columns: _,
                x: _,
                picker,
            } => picker,
            State::PickColorBy { x: _, y: _, picker } => picker,
        }
    }
}
