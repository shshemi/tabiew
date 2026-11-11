use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Gauge, Paragraph, Widget},
};

use crate::{
    misc::config::theme,
    net::remote_load::RemoteLoad,
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
    },
};

pub struct DownloadNotification {
    dl: RemoteLoad,
    title: String,
}

impl DownloadNotification {
    pub fn new(title: String, dl: RemoteLoad) -> Self {
        DownloadNotification { title, dl }
    }

    pub fn is_running(&self) -> bool {
        self.dl.running()
    }

    pub fn into_remote_load(self) -> RemoteLoad {
        self.dl
    }
}

impl Component for DownloadNotification {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: crate::tui::component::FocusState,
    ) {
        render_progress(
            &self.title,
            self.dl.info().percent(),
            "Downloading...",
            area,
            buf,
        );
    }
}

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
