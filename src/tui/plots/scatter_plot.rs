use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyModifiers};
use itertools::Itertools;
use polars::{datatypes::DataType, frame::DataFrame};
use ratatui::{
    layout::{Alignment, Constraint},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Block, Chart, Clear, Dataset, GraphType, LegendPosition, Padding, Widget},
};

use crate::{
    AppResult,
    collections::ragged_vec::RaggedVec,
    handler::message::Message,
    misc::config::{config, theme},
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        layouts::plot::PlotLayout,
        misc::any_value_formatter::AnyValueFormatter,
    },
};

#[derive(Debug)]
pub struct ScatterPlot {
    data: RaggedVec<(f64, f64)>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    x_label: String,
    y_label: String,
    groups: Option<Vec<String>>,
}

impl Component for ScatterPlot {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let area = PlotLayout::default().area(buf.area);
        Clear.render(area, buf);
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
                Block::app_default()
                    .app_title("Scatter Plot")
                    .title_alignment(Alignment::Center)
                    .padding(Padding::new(1, 2, 0, 0)),
            )
            .legend_position(Some(LegendPosition::TopRight))
            .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)));

        chart.render(area, buf);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::PaneDismissModal.enqueue();
                true
            }
            (KeyCode::Enter, KeyModifiers::NONE) => true,
            _ => false,
        }
    }
}

pub struct ScatterPlotBuilder<'a> {
    df: &'a DataFrame,
    x_label: &'a str,
    y_label: &'a str,
    group_by: Option<&'a str>,
}

impl<'a> ScatterPlotBuilder<'a> {
    pub fn new(df: &'a DataFrame, x_label: &'a str, y_label: &'a str) -> Self {
        ScatterPlotBuilder {
            df,
            x_label,
            y_label,
            group_by: None,
        }
    }

    pub fn with_group(self, group_by: &'a str) -> Self {
        ScatterPlotBuilder {
            group_by: Some(group_by),
            ..self
        }
    }

    pub fn build(self) -> AppResult<ScatterPlot> {
        if let Some(group_by) = self.group_by {
            let (data, groups) =
                scatter_plot_data_grouped(self.df, self.x_label, self.y_label, group_by)?;
            let [x_bounds, y_bounds] = data_bounds(&data)?;
            Ok(ScatterPlot {
                data,
                x_bounds,
                y_bounds,
                x_label: self.x_label.to_owned(),
                y_label: self.y_label.to_owned(),
                groups: Some(groups),
            })
        } else {
            let data = scatter_plot_data(self.df, self.x_label, self.y_label)?;
            let [x_bounds, y_bounds] = data_bounds(&data)?;
            Ok(ScatterPlot {
                data,
                x_bounds,
                y_bounds,
                x_label: self.x_label.to_owned(),
                y_label: self.y_label.to_owned(),
                groups: None,
            })
        }
    }
}

fn data_bounds(data: &RaggedVec<(f64, f64)>) -> AppResult<[[f64; 2]; 2]> {
    data.iter()
        .flat_map(|v| v.iter())
        .fold(None, |bounds: Option<[[f64; 2]; 2]>, &(x, y)| {
            let [[x_min, x_max], [y_min, y_max]] = bounds.unwrap_or([[x, x], [y, y]]);
            Some([[x_min.min(x), x_max.max(x)], [y_min.min(y), y_max.max(y)]])
        })
        .ok_or(anyhow!("Empty dimension(s)"))
}

fn scatter_plot_data(
    df: &DataFrame,
    x_label: &str,
    y_label: &str,
) -> AppResult<RaggedVec<(f64, f64)>> {
    Ok(df
        .column(x_label)?
        .cast(&DataType::Float64)?
        .f64()?
        .iter()
        .zip(df.column(y_label)?.cast(&DataType::Float64)?.f64()?.iter())
        .filter_map(|(x, y)| Some((x?, y?)))
        .collect())
}

#[allow(clippy::type_complexity)]
fn scatter_plot_data_grouped(
    df: &DataFrame,
    x_label: &str,
    y_label: &str,
    group_by: &str,
) -> AppResult<(RaggedVec<(f64, f64)>, Vec<String>)> {
    let fp_prec = config().fp_precision();
    let mut groups = Vec::new();
    let mut data = RaggedVec::new();
    for (name, df) in df
        .partition_by(vec![group_by], true)?
        .into_iter()
        .map(|df| {
            let name = df
                .column(group_by)
                .and_then(|column| column.get(0))
                .map(|val| {
                    AnyValueFormatter::new(fp_prec)
                        .into_single_line(val)
                        .into_owned()
                })
                .unwrap_or("null".to_owned());
            (name, df)
        })
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
    {
        groups.push(name);
        data.push(scatter_plot_data(&df, x_label, y_label)?);
    }
    Ok((data, groups))
}
