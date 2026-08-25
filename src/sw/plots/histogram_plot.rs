use std::borrow::Cow;

use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Direction, Rect},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    misc::config::theme,
    sw::{
        app_default::AppDefault,
        buffer_ext::BufferExt,
        rect_ext::RectExt,
        widgets::tag_line::{Tag, TagLine},
    },
};

const MAX_LABEL_WIDTH: usize = 24;

#[derive(Debug)]
pub struct HistogramPlotState {
    offset: usize,
    bars: Vec<Bar<'static>>,
    max_value: u64,
}

impl HistogramPlotState {
    pub fn new(data: Vec<(String, u64)>) -> Self {
        Self {
            offset: 0,
            max_value: data
                .iter()
                .map(|(_, value)| *value)
                .max()
                .unwrap_or_default(),
            bars: bars_from_data(data),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn max_value(&self) -> u64 {
        self.max_value
    }

    pub fn len(&self) -> usize {
        self.bars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    pub fn scroll_up(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.offset = self.offset.saturating_add(1);
    }
}

#[derive(Debug)]
pub struct HistogramPlot<'a> {
    title: Cow<'a, str>,
}

impl<'a> HistogramPlot<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for HistogramPlot<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Histogram Plot"),
        }
    }
}

impl StatefulWidget for HistogramPlot<'_> {
    type State = HistogramPlotState;

    fn render(self, _area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = buf.area.plot();
        buf.clear(area);

        let block = Block::app_default()
            .title(self.title)
            .title_alignment(Alignment::Center)
            .title_bottom(
                TagLine::mono_color()
                    .centered()
                    .tag(Tag::new(" Scroll Up ", " Shift+K | Shift+\u{2191} "))
                    .tag(Tag::new(" Scroll Down ", " Shift+J | Shift+\u{2193} ")),
            );
        let inner = block.inner(area);
        block.render(area, buf);

        state.offset = state
            .offset
            .min(state.bars.len().saturating_sub(inner.height as usize));
        let end = state
            .offset
            .saturating_add(inner.height as usize)
            .min(state.bars.len());

        BarChart::default()
            .style(theme().text())
            .bar_width(1)
            .max(state.max_value)
            .direction(Direction::Horizontal)
            .bar_gap(0)
            .data(BarGroup::default().bars(&state.bars[state.offset..end]))
            .render(inner, buf);
    }
}

fn bars_from_data(data: Vec<(String, u64)>) -> Vec<Bar<'static>> {
    let label_len = data
        .iter()
        .map(|(label, _)| label.trim().width())
        .max()
        .unwrap_or_default()
        .min(MAX_LABEL_WIDTH);
    let value_len = data
        .iter()
        .map(|(_, value)| value.to_string().len())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn data(count: usize) -> Vec<(String, u64)> {
        (0..count)
            .map(|idx| (format!("bucket{idx}"), idx as u64 * 10))
            .collect()
    }

    fn render(state: &mut HistogramPlotState, plot: HistogramPlot) -> Buffer {
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
        fn the_largest_value_becomes_the_chart_maximum() {
            let state = HistogramPlotState::new(data(4));

            assert_eq!(state.max_value(), 30);
        }

        #[test]
        fn an_empty_dataset_has_no_bars() {
            let state = HistogramPlotState::new(Vec::new());

            assert!(state.is_empty());
            assert_eq!(state.max_value(), 0);
        }

        #[test]
        fn every_bucket_becomes_a_bar() {
            let state = HistogramPlotState::new(data(4));

            assert_eq!(state.len(), 4);
        }

        #[test]
        fn it_starts_at_the_top() {
            assert_eq!(HistogramPlotState::new(data(4)).offset(), 0);
        }

        #[test]
        fn scrolling_up_saturates_at_the_top() {
            let mut state = HistogramPlotState::new(data(4));
            state.scroll_up();

            assert_eq!(state.offset(), 0);
        }

        #[test]
        fn scrolling_down_advances_until_a_render_clamps_it() {
            let mut state = HistogramPlotState::new(data(4));
            state.scroll_down();
            state.scroll_down();

            assert_eq!(state.offset(), 2);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn defaults_to_a_histogram_plot_title() {
            let buf = render(
                &mut HistogramPlotState::new(data(3)),
                HistogramPlot::default(),
            );

            assert!(content(&buf).contains("Histogram Plot"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut HistogramPlotState::new(data(3)),
                HistogramPlot::default().title("Counts"),
            );

            assert!(content(&buf).contains("Counts"));
        }

        #[test]
        fn renders_the_bucket_labels() {
            let buf = render(
                &mut HistogramPlotState::new(data(3)),
                HistogramPlot::default(),
            );

            let content = content(&buf);
            assert!(content.contains("bucket0"));
            assert!(content.contains("bucket2"));
        }

        #[test]
        fn renders_the_scroll_hints() {
            let buf = render(
                &mut HistogramPlotState::new(data(3)),
                HistogramPlot::default(),
            );

            let content = content(&buf);
            assert!(content.contains("Scroll Up"));
            assert!(content.contains("Scroll Down"));
        }

        #[test]
        fn content_that_fits_is_never_scrolled() {
            let mut state = HistogramPlotState::new(data(3));
            for _ in 0..10 {
                state.scroll_down();
            }
            render(&mut state, HistogramPlot::default());

            assert_eq!(state.offset(), 0);
        }

        #[test]
        fn scrolling_stops_at_the_last_page() {
            let mut state = HistogramPlotState::new(data(80));
            for _ in 0..200 {
                state.scroll_down();
            }
            render(&mut state, HistogramPlot::default());
            let bottom = state.offset();
            assert!(bottom > 0);

            state.scroll_down();
            render(&mut state, HistogramPlot::default());

            assert_eq!(state.offset(), bottom);
        }

        #[test]
        fn scrolling_shows_later_buckets() {
            let mut state = HistogramPlotState::new(data(80));
            let before = content(&render(&mut state, HistogramPlot::default()));

            for _ in 0..200 {
                state.scroll_down();
            }
            let after = content(&render(&mut state, HistogramPlot::default()));

            assert_ne!(before, after);
            assert!(!before.contains("bucket79"));
            assert!(after.contains("bucket79"));
        }

        #[test]
        fn leaves_the_margin_around_the_plot_alone() {
            let buf = render(
                &mut HistogramPlotState::new(data(3)),
                HistogramPlot::default(),
            );

            assert_eq!(buf[(0, 0)].symbol(), " ");
            assert_eq!(buf[(99, 29)].symbol(), " ");
        }

        #[test]
        fn an_empty_dataset_renders_without_panicking() {
            let mut state = HistogramPlotState::new(Vec::new());
            render(&mut state, HistogramPlot::default());

            assert_eq!(state.offset(), 0);
        }

        #[test]
        fn a_tiny_area_renders_without_panicking() {
            let area = Rect::new(0, 0, 8, 4);
            let mut buf = Buffer::empty(area);
            HistogramPlot::default().render(area, &mut buf, &mut HistogramPlotState::new(data(3)));
        }
    }
}
