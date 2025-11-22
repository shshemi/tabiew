use anyhow::anyhow;
use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Constraint, Margin},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Chart, Clear, Dataset, GraphType, LegendPosition, Padding, Widget},
};

use crate::{
    AppResult,
    misc::{globals::theme, jagged_vec::JaggedVec},
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct ScatterPlot {
    data: JaggedVec<(f64, f64)>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    x_label: String,
    y_label: String,
    groups: Option<Vec<String>>,
}

impl ScatterPlot {
    pub fn new(x_label: String, y_label: String, data: JaggedVec<(f64, f64)>) -> AppResult<Self> {
        let [x_bounds, y_bounds] = data
            .iter()
            .flat_map(|v| v.iter())
            .fold(None, |bounds, p| {
                let bounds = bounds.unwrap_or([[p.0, p.0], [p.1, p.1]]);
                Some([
                    [bounds[0][0].min(p.0), bounds[0][1].max(p.0)],
                    [bounds[1][0].min(p.1), bounds[1][1].max(p.1)],
                ])
            })
            .ok_or(anyhow!("Empty dimension"))?;
        Ok(Self {
            data,
            x_bounds,
            y_bounds,
            x_label,
            y_label,
            groups: None,
        })
    }

    pub fn with_groups(self, groups: impl Into<Option<Vec<String>>>) -> Self {
        Self {
            groups: groups.into(),
            ..self
        }
    }
}

impl Component for ScatterPlot {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let area = buf.area.inner(Margin::new(10, 10));
        Widget::render(Clear, area, buf);
        let ds = self
            .data
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let ds = Dataset::default()
                    .marker(Marker::Dot)
                    .graph_type(GraphType::Scatter)
                    .style(theme().graph(i))
                    .data(v);
                if let Some(g) = &self.groups {
                    ds.name(g[i].as_str())
                } else {
                    ds
                }
            })
            .collect_vec();

        let chart = Chart::new(ds)
            .x_axis(
                Axis::default()
                    .title(Span::styled(&self.x_label, theme().text()))
                    .bounds(self.x_bounds)
                    .style(theme().text())
                    .labels(
                        self.x_bounds
                            .map(|f| Span::styled(format!("{f:.2}"), theme().text())),
                    ),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled(&self.y_label, theme().text()))
                    .bounds(self.y_bounds)
                    .style(theme().text())
                    .labels(
                        self.y_bounds
                            .map(|f| Span::styled(format!("{f:.2}"), theme().text())),
                    ),
            )
            .style(theme().text())
            .block(
                Block::default()
                    .title("Scatter Plot")
                    .title_alignment(Alignment::Center)
                    .padding(Padding::new(1, 2, 0, 0))
                    .into_widget(),
            )
            .legend_position(Some(LegendPosition::TopRight))
            .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)));
        // .hidden_legend_constraints((Constraint::Min(0), Constraint::Ratio(1, 2)));

        chart.render(area, buf);
    }
}

// impl StatefulWidget for ScatterPlot {
//     type State = ScatterPlotState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         Widget::render(Clear, area, buf);
//         let ds = state
//             .data
//             .iter()
//             .enumerate()
//             .map(|(i, v)| {
//                 let ds = Dataset::default()
//                     .marker(Marker::Dot)
//                     .graph_type(GraphType::Scatter)
//                     .style(theme().graph(i))
//                     .data(v);
//                 if let Some(g) = &state.groups {
//                     ds.name(g[i].as_str())
//                 } else {
//                     ds
//                 }
//             })
//             .collect_vec();

//         let chart = Chart::new(ds)
//             .x_axis(
//                 Axis::default()
//                     .title(Span::styled(&state.x_label, theme().text()))
//                     .bounds(state.x_bounds)
//                     .style(theme().text())
//                     .labels(
//                         state
//                             .x_bounds
//                             .map(|f| Span::styled(format!("{f:.2}"), theme().text())),
//                     ),
//             )
//             .y_axis(
//                 Axis::default()
//                     .title(Span::styled(&state.y_label, theme().text()))
//                     .bounds(state.y_bounds)
//                     .style(theme().text())
//                     .labels(
//                         state
//                             .y_bounds
//                             .map(|f| Span::styled(format!("{f:.2}"), theme().text())),
//                     ),
//             )
//             .style(theme().text())
//             .block(
//                 Block::default()
//                     .title("Scatter Plot")
//                     .title_alignment(Alignment::Center)
//                     .padding(Padding::new(1, 2, 0, 0))
//                     .into_widget(),
//             )
//             .legend_position(Some(LegendPosition::TopRight))
//             .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)));
//         // .hidden_legend_constraints((Constraint::Min(0), Constraint::Ratio(1, 2)));

//         chart.render(area, buf);
//     }
// }
