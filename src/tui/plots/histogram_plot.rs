use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Direction, Size},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Clear, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{misc::globals::theme, tui::widgets::block::Block};

#[derive(Debug, Default)]
pub struct HistogramPlot;

#[derive(Debug)]
pub struct HistogramPlotState {
    data: Vec<(String, u64)>,
    scroll_view: ScrollViewState,
}

impl HistogramPlotState {
    pub fn new(data: Vec<(String, u64)>) -> Self {
        Self {
            data,
            scroll_view: Default::default(),
        }
    }

    pub fn bucket_count(&self) -> usize {
        self.data.len()
    }

    pub fn scroll_up(&mut self) {
        self.scroll_view.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_view.scroll_down();
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
        let area = {
            let blk = Block::default()
                .title("Histogram Plot")
                .title_alignment(Alignment::Center);
            let new_area = blk.inner(area);
            blk.render(area, buf);
            new_area
        };

        let lab_len = state
            .data
            .iter()
            .map(|(l, _)| l.len())
            .max()
            .unwrap_or_default();
        let val_len = state
            .data
            .iter()
            .map(|(_, v)| v.to_string().len())
            .max()
            .unwrap_or_default();
        let bars = state
            .data
            .iter()
            .enumerate()
            .map(|(idx, (label, value))| {
                Bar::default()
                    .value(*value)
                    .text_value(format!("{value:>val_len$} "))
                    .label(Line::styled(
                        format!("{label:>lab_len$} "),
                        theme().graph(idx),
                    ))
                    .style(theme().graph(idx))
            })
            .collect_vec();

        let mut scroll_view = ScrollView::new(Size::new(area.width, bars.len() as u16))
            .scrollbars_visibility(ScrollbarVisibility::Never);
        if !bars.is_empty() {
            let chart = BarChart::default()
                .style(theme().text())
                .bar_width(1)
                .direction(Direction::Horizontal)
                .bar_gap(0)
                .data(BarGroup::default().bars(&bars));
            scroll_view.render_widget(chart, scroll_view.area());
        }
        scroll_view.render(area, buf, &mut state.scroll_view);
    }
}
