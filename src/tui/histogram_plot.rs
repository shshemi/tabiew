use std::ops::Div;

use itertools::Itertools;
use ratatui::{
    layout::Alignment,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Clear, StatefulWidget, Widget},
};

use crate::misc::globals::theme;

#[derive(Debug, Default)]
pub struct HistogramPlot;

#[derive(Debug)]
pub struct HistogramPlotState {
    data: Vec<(String, u64)>,
}

impl HistogramPlotState {
    pub fn new(data: Vec<(String, u64)>) -> Self {
        Self { data }
    }

    pub fn bucket_count(&self) -> usize {
        self.data.len()
    }
}

impl StatefulWidget for HistogramPlot {
    type State = HistogramPlotState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Widget::render(Clear, area, buf);

        let bars = state
            .data
            .iter()
            .take(area.width.saturating_sub(2) as usize)
            .enumerate()
            .map(|(idx, (label, value))| {
                Bar::default()
                    .value(*value)
                    // .label(Line::raw(label).style(theme().text()))
                    .label(Line::styled(label, theme().graph(idx)))
                    .style(theme().graph(idx))
            })
            .collect_vec();

        if !bars.is_empty() {
            let chart = BarChart::default()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .style(theme().block())
                        .title("Histogram Plot")
                        .title_alignment(Alignment::Center),
                )
                .bar_width(
                    area.width
                        .saturating_sub(2)
                        .div(bars.len().min(24) as u16)
                        .max(5),
                )
                .bar_gap(0)
                .data(BarGroup::default().bars(&bars));
            chart.render(area, buf);
        }
    }
}
