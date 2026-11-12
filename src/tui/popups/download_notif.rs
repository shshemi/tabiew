use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Gauge, Paragraph, Widget},
};

use crate::{
    misc::config::theme,
    tui::app_default::{AppDefault, AppTitle},
};

pub fn idle(title: &str) -> String {
    format!(
        "{}...",
        title.split_whitespace().next().unwrap_or("Working")
    )
}

pub fn render_progress(
    title: &str,
    percent: Option<u16>,
    waiting: &str,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::app_default().app_title(title);
    match percent {
        Some(percent) => Gauge::default()
            .block(block)
            .gauge_style(theme().block())
            .percent(percent.min(100))
            .render(area, buf),
        None => Paragraph::new(waiting).block(block).render(area, buf),
    }
}
