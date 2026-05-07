use ratatui::widgets::{Gauge, Paragraph, Widget};

use crate::{
    misc::{config::theme, download::DownloadAndRead},
    tui::{component::Component, widgets::block::Block},
};

pub struct DownloadNotification {
    dl: DownloadAndRead,
    title: String,
}

impl DownloadNotification {
    pub fn new(title: String, dl: DownloadAndRead) -> Self {
        DownloadNotification { title, dl }
    }

    pub fn is_running(&self) -> bool {
        self.dl.running()
    }

    pub fn into_downloader(self) -> DownloadAndRead {
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
        if let Some(percent) = self.dl.info().percent() {
            Gauge::default()
                .block(Block::default().title(self.title.as_str()).into_widget())
                .gauge_style(theme().block())
                .percent(percent)
                .render(area, buf);
        } else {
            Paragraph::new("Downloading...")
                .block(Block::default().title(self.title.as_str()).into_widget())
                .render(area, buf);
        }
    }
}
