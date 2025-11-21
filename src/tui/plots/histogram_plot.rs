use crossterm::event::KeyCode;
use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Direction, Margin, Size},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Clear, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{
    misc::globals::theme,
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct HistogramPlot {
    data: Vec<(String, u64)>,
    scroll_view: ScrollViewState,
}

impl HistogramPlot {
    pub fn new(data: Vec<(String, u64)>) -> Self {
        Self {
            data,
            scroll_view: Default::default(),
        }
    }

    pub fn bucket_count(&self) -> usize {
        self.data.len()
    }
}

impl Component for HistogramPlot {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let area = buf.area.inner(Margin::new(10, 10));
        Widget::render(Clear, area, buf);
        let area = {
            let blk = Block::default()
                .title("Histogram Plot")
                .title_alignment(Alignment::Center);
            let new_area = blk.inner(area);
            blk.render(area, buf);
            new_area
        };

        let lab_len = self
            .data
            .iter()
            .map(|(l, _)| l.len())
            .max()
            .unwrap_or_default();
        let val_len = self
            .data
            .iter()
            .map(|(_, v)| v.to_string().len())
            .max()
            .unwrap_or_default();
        let bars = self
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
        scroll_view.render(area, buf, &mut self.scroll_view);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_view.scroll_up();
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll_view.scroll_down();
                true
            }
            _ => false,
        }
    }
}

// impl StatefulWidget for HistogramPlot {
//     type State = HistogramPlotState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         Widget::render(Clear, area, buf);
//         let area = {
//             let blk = Block::default()
//                 .title("Histogram Plot")
//                 .title_alignment(Alignment::Center);
//             let new_area = blk.inner(area);
//             blk.render(area, buf);
//             new_area
//         };

//         let lab_len = state
//             .data
//             .iter()
//             .map(|(l, _)| l.len())
//             .max()
//             .unwrap_or_default();
//         let val_len = state
//             .data
//             .iter()
//             .map(|(_, v)| v.to_string().len())
//             .max()
//             .unwrap_or_default();
//         let bars = state
//             .data
//             .iter()
//             .enumerate()
//             .map(|(idx, (label, value))| {
//                 Bar::default()
//                     .value(*value)
//                     .text_value(format!("{value:>val_len$} "))
//                     .label(Line::styled(
//                         format!("{label:>lab_len$} "),
//                         theme().graph(idx),
//                     ))
//                     .style(theme().graph(idx))
//             })
//             .collect_vec();

//         let mut scroll_view = ScrollView::new(Size::new(area.width, bars.len() as u16))
//             .scrollbars_visibility(ScrollbarVisibility::Never);
//         if !bars.is_empty() {
//             let chart = BarChart::default()
//                 .style(theme().text())
//                 .bar_width(1)
//                 .direction(Direction::Horizontal)
//                 .bar_gap(0)
//                 .data(BarGroup::default().bars(&bars));
//             scroll_view.render_widget(chart, scroll_view.area());
//         }
//         scroll_view.render(area, buf, &mut state.scroll_view);
//     }
// }
