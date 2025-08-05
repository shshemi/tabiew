use std::ops::Add;

use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Direction},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Clear, StatefulWidget, Widget},
};

use crate::misc::globals::theme;

#[derive(Debug, Default)]
pub struct HistogramPlot;

#[derive(Debug)]
pub struct HistogramPlotState {
    data: Vec<(String, u64)>,
    scroll: usize,
}

impl HistogramPlotState {
    pub fn new(data: Vec<(String, u64)>) -> Self {
        Self { data, scroll: 0 }
    }

    pub fn bucket_count(&self) -> usize {
        self.data.len()
    }

    pub fn scroll_up(&mut self) {
        //
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        //
        self.scroll = self.scroll.add(1);
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

        let height = area.height.saturating_sub(2).into();
        state.scroll = state.scroll.min(state.data.len().saturating_sub(height));

        let bars = state
            .data
            .iter()
            .skip(state.scroll)
            .take(height)
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
                .bar_width(1)
                .direction(Direction::Horizontal)
                .bar_gap(0)
                .data(BarGroup::default().bars(&bars));
            chart.render(area, buf);
        }
    }
}
