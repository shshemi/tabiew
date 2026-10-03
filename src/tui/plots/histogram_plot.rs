use std::ops::Add;

use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyModifiers};
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{ChunkAgg, DataType, SeriesMethods},
};
use ratatui::{
    layout::{Alignment, Direction},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Clear, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    AppResult,
    handler::message::Message,
    misc::config::{config, theme},
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
        layouts::plot::PlotLayout,
        misc::any_value_formatter::AnyValueFormatter,
        tag_line::{Tag, TagLine},
    },
};

#[derive(Debug)]
pub struct HistogramPlot {
    offset: usize,
    bars: Vec<Bar<'static>>,
    max_value: u64,
}

impl HistogramPlot {
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
        let area = PlotLayout::default().area(buf.area);
        Clear.render(area, buf);
        let area = {
            let blk = Block::app_default()
                .app_title("Histogram Plot")
                .title_alignment(Alignment::Center)
                .title_bottom(TagLine::default().mono_color().centered().tag(Tag::new(
                    icons::HEIGHT.str("Scroll"),
                    "Shift+\u{2193}\u{2191}/JK",
                )));
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
            (KeyCode::Up, KeyModifiers::SHIFT) | (KeyCode::Char('K'), KeyModifiers::SHIFT) => {
                self.scroll_up();
                true
            }
            (KeyCode::Down, KeyModifiers::SHIFT) | (KeyCode::Char('J'), KeyModifiers::SHIFT) => {
                self.scroll_down();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::PaneDismissModal.enqueue();
                true
            }
            (KeyCode::Enter, KeyModifiers::NONE) => true,
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

pub struct HistogramPlotBuilder<'a> {
    df: &'a DataFrame,
    column: &'a str,
    buckets: usize,
}

impl<'a> HistogramPlotBuilder<'a> {
    pub fn new(df: &'a DataFrame, column: &'a str, buckets: usize) -> Self {
        HistogramPlotBuilder {
            df,
            column,
            buckets,
        }
    }

    pub fn build(self) -> AppResult<HistogramPlot> {
        let data = histogram_plot_data(self.df, self.column, self.buckets)?;
        Ok(HistogramPlot {
            offset: 0,
            max_value: data.iter().map(|(_, v)| *v).max().unwrap_or_default(),
            bars: bars_from_data(data),
        })
    }
}

fn histogram_plot_data(
    df: &DataFrame,
    col_name: &str,
    buckets: usize,
) -> AppResult<Vec<(String, u64)>> {
    let col = df.column(col_name)?;
    match col.dtype() {
        DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64
        | DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::Int128
        | DataType::Float32
        | DataType::Float64
        | DataType::Decimal(_, _) => continues_histogram(
            col.as_materialized_series()
                .value_counts(true, true, "value".into(), false)?,
            buckets,
        ),
        DataType::Boolean | DataType::String => discrete_histogram(
            col.as_materialized_series()
                .value_counts(true, true, "value".into(), false)?,
        ),
        _ => Err(anyhow!("Unsupported column type"))?,
    }
}

fn discrete_histogram(mut counts: DataFrame) -> AppResult<Vec<(String, u64)>> {
    let fp_precision = config().fp_precision();
    counts.rechunk_mut();
    Ok(counts[0]
        .as_materialized_series()
        .iter()
        .map(|val| {
            AnyValueFormatter::new(fp_precision)
                .into_single_line(val)
                .into_owned()
        })
        .zip(counts[1].as_materialized_series().u32()?.iter())
        .map(|(v, c)| (v, c.unwrap_or_default() as u64))
        .collect_vec())
}

fn continues_histogram(counts: DataFrame, buckets: usize) -> AppResult<Vec<(String, u64)>> {
    let casted = counts[0].cast(&DataType::Float64)?;
    let arr = casted.f64()?;
    let (min, max) = arr.min_max().ok_or(anyhow!("No value found"))?;
    let width = (max - min) / (buckets as f64);
    let counts = arr
        .iter()
        .flatten()
        .zip(counts[1].as_materialized_series().u32()?.iter().flatten())
        .fold(vec![0; buckets], |mut buckets, (v, c)| {
            let idx = (((v - min) / width) as usize).min(buckets.len().saturating_sub(1));
            buckets[idx] += c;
            buckets
        });
    let label_len = format!("{max:.2}").len();
    Ok(counts
        .into_iter()
        .enumerate()
        .map(|(idx, r)| {
            let start = (idx as f64) * width + min;
            let end = (idx.add(1) as f64) * width + min;
            (
                format!(" {start:>w$.2} - {end:>w$.2}", w = label_len),
                r as u64,
            )
        })
        .collect())
}
