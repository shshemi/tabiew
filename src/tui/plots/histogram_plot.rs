use crossterm::event::{KeyCode, KeyModifiers};
use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Direction, Margin},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Clear, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    handler::message::Message,
    misc::globals::theme,
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct HistogramPlot {
    offset: usize,
    bars: Vec<Bar<'static>>,
    max_value: u64,
}

impl HistogramPlot {
    pub fn new(data: Vec<(String, u64)>) -> Self {
        Self {
            offset: 0,
            max_value: data.first().map(|(_, v)| *v).unwrap_or_default(),
            bars: bars_from_data(data),
        }
    }

    fn scroll_up(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        self.offset = self.offset.saturating_add(1);
    }
}

impl Component for HistogramPlot {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let area = buf.area.inner(Margin::new(7, 3));
        Widget::render(Clear, area, buf);
        let area = {
            let blk = Block::default()
                .title("Histogram Plot")
                .title_alignment(Alignment::Center);
            let new_area = blk.inner(area);
            blk.render(area, buf);
            new_area
        };

        self.offset = self
            .offset
            .min(self.bars.len().saturating_sub(area.height as usize));

        let end = self
            .offset
            .saturating_add(area.height as usize)
            .min(self.bars.len());

        let chart = BarChart::default()
            .style(theme().text())
            .bar_width(1)
            .max(self.max_value)
            .direction(Direction::Horizontal)
            .bar_gap(0)
            .data(BarGroup::default().bars(&self.bars[self.offset..end]));
        chart.render(area, buf);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
                self.scroll_up();
                true
            }
            (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
                self.scroll_down();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::PaneDismissModal.enqueue();
                true
            }
            _ => false,
        }
    }
}

fn bars_from_data(data: Vec<(String, u64)>) -> Vec<Bar<'static>> {
    let label_len = data
        .iter()
        .map(|(l, _)| l.trim().width())
        .max()
        .unwrap_or_default()
        .min(24);
    let value_len = data
        .iter()
        .map(|(_, v)| v.to_string().len())
        .max()
        .unwrap_or_default();
    data.iter()
        .enumerate()
        .map(|(idx, (label, value))| {
            let label = label.trim().chars().take(label_len).collect::<String>();
            Bar::default()
                .value(*value)
                .text_value(format!("{value:>value_len$} "))
                .label(Line::styled(
                    format!("{label:>label_len$}"),
                    theme().graph(idx),
                ))
                .style(theme().graph(idx))
        })
        .collect_vec()
}
