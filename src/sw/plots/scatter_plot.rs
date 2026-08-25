use std::borrow::Cow;

use anyhow::anyhow;
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    symbols::Marker,
    text::Span,
    widgets::{
        Axis, Block, Chart, Dataset, GraphType, LegendPosition, Padding, StatefulWidget, Widget,
    },
};

use crate::{
    AppResult,
    misc::{config::theme, ragged_vec::RaggedVec},
    sw::{app_default::AppDefault, buffer_ext::BufferExt, rect_ext::RectExt},
};

#[derive(Debug)]
pub struct ScatterPlotState {
    data: RaggedVec<(f64, f64)>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    x_label: String,
    y_label: String,
    groups: Option<Vec<String>>,
}

impl ScatterPlotState {
    pub fn new(x_label: String, y_label: String, data: RaggedVec<(f64, f64)>) -> AppResult<Self> {
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

    pub fn x_bounds(&self) -> [f64; 2] {
        self.x_bounds
    }

    pub fn y_bounds(&self) -> [f64; 2] {
        self.y_bounds
    }

    pub fn x_label(&self) -> &str {
        &self.x_label
    }

    pub fn y_label(&self) -> &str {
        &self.y_label
    }

    pub fn groups(&self) -> Option<&[String]> {
        self.groups.as_deref()
    }
}

#[derive(Debug)]
pub struct ScatterPlot<'a> {
    title: Cow<'a, str>,
}

impl<'a> ScatterPlot<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for ScatterPlot<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Scatter Plot"),
        }
    }
}

impl StatefulWidget for ScatterPlot<'_> {
    type State = ScatterPlotState;

    fn render(self, _area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = buf.area.plot();
        buf.clear(area);

        let datasets = state
            .data
            .iter()
            .enumerate()
            .map(|(idx, points)| {
                let dataset = Dataset::default()
                    .marker(Marker::Dot)
                    .graph_type(GraphType::Scatter)
                    .style(theme().graph(idx))
                    .data(points);
                match state.groups.as_ref().and_then(|groups| groups.get(idx)) {
                    Some(name) => dataset.name(name.as_str()),
                    None => dataset,
                }
            })
            .collect_vec();

        Chart::new(datasets)
            .x_axis(axis(&state.x_label, state.x_bounds))
            .y_axis(axis(&state.y_label, state.y_bounds))
            .style(theme().text())
            .block(
                Block::app_default()
                    .title(self.title)
                    .title_alignment(Alignment::Center)
                    .padding(Padding::new(1, 2, 0, 0)),
            )
            .legend_position(Some(LegendPosition::TopRight))
            .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)))
            .render(area, buf);
    }
}

fn axis<'a>(label: &'a str, bounds: [f64; 2]) -> Axis<'a> {
    Axis::default()
        .title(Span::styled(label, theme().text()))
        .bounds(bounds)
        .style(theme().text())
        .labels(bounds.map(|f| Span::styled(format!("{f:.2}"), theme().text())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_group(points: Vec<(f64, f64)>) -> RaggedVec<(f64, f64)> {
        let mut data = RaggedVec::new();
        data.push(points);
        data
    }

    fn two_groups() -> RaggedVec<(f64, f64)> {
        let mut data = RaggedVec::new();
        data.push(vec![(0.0, 0.0), (1.0, 1.0)]);
        data.push(vec![(4.0, 8.0)]);
        data
    }

    fn state() -> ScatterPlotState {
        ScatterPlotState::new("width".to_owned(), "height".to_owned(), two_groups()).unwrap()
    }

    fn render(state: &mut ScatterPlotState, plot: ScatterPlot) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        plot.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn bounds_span_every_point_in_every_group() {
            let state = state();

            assert_eq!(state.x_bounds(), [0.0, 4.0]);
            assert_eq!(state.y_bounds(), [0.0, 8.0]);
        }

        #[test]
        fn a_single_point_gives_degenerate_bounds() {
            let state = ScatterPlotState::new(
                "x".to_owned(),
                "y".to_owned(),
                single_group(vec![(2.0, 5.0)]),
            )
            .unwrap();

            assert_eq!(state.x_bounds(), [2.0, 2.0]);
            assert_eq!(state.y_bounds(), [5.0, 5.0]);
        }

        #[test]
        fn negative_values_widen_the_lower_bound() {
            let state = ScatterPlotState::new(
                "x".to_owned(),
                "y".to_owned(),
                single_group(vec![(-3.0, -7.0), (1.0, 1.0)]),
            )
            .unwrap();

            assert_eq!(state.x_bounds(), [-3.0, 1.0]);
            assert_eq!(state.y_bounds(), [-7.0, 1.0]);
        }

        #[test]
        fn an_empty_dataset_is_rejected() {
            let empty = ScatterPlotState::new("x".to_owned(), "y".to_owned(), RaggedVec::new());

            assert!(empty.is_err());
        }

        #[test]
        fn the_labels_are_kept() {
            let state = state();

            assert_eq!(state.x_label(), "width");
            assert_eq!(state.y_label(), "height");
        }

        #[test]
        fn groups_are_absent_by_default() {
            assert!(state().groups().is_none());
        }

        #[test]
        fn groups_can_be_attached() {
            let state = state().with_groups(vec!["a".to_owned(), "b".to_owned()]);

            assert_eq!(
                state.groups(),
                Some(["a".to_owned(), "b".to_owned()].as_slice())
            );
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn defaults_to_a_scatter_plot_title() {
            let buf = render(&mut state(), ScatterPlot::default());

            assert!(content(&buf).contains("Scatter Plot"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(&mut state(), ScatterPlot::default().title("Points"));

            assert!(content(&buf).contains("Points"));
        }

        #[test]
        fn renders_the_axis_labels() {
            let buf = render(&mut state(), ScatterPlot::default());

            let content = content(&buf);
            assert!(content.contains("width"));
            assert!(content.contains("height"));
        }

        #[test]
        fn renders_the_axis_bounds() {
            let buf = render(&mut state(), ScatterPlot::default());

            let content = content(&buf);
            assert!(content.contains("0.00"));
            assert!(content.contains("4.00"));
            assert!(content.contains("8.00"));
        }

        #[test]
        fn group_names_appear_in_the_legend() {
            let mut state = state().with_groups(vec!["first".to_owned(), "second".to_owned()]);
            let buf = render(&mut state, ScatterPlot::default());

            let content = content(&buf);
            assert!(content.contains("first"));
            assert!(content.contains("second"));
        }

        #[test]
        fn leaves_the_margin_around_the_plot_alone() {
            let buf = render(&mut state(), ScatterPlot::default());

            assert_eq!(buf[(0, 0)].symbol(), " ");
            assert_eq!(buf[(99, 29)].symbol(), " ");
        }

        #[test]
        fn a_tiny_area_renders_without_panicking() {
            let area = Rect::new(0, 0, 8, 4);
            let mut buf = Buffer::empty(area);
            ScatterPlot::default().render(area, &mut buf, &mut state());
        }
    }
}
