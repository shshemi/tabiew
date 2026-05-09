use ratatui::widgets::{Gauge, Paragraph, Widget};

use crate::{
    misc::{config::theme, remote_load::RemoteLoad},
    tui::{component::Component, widgets::block::Block},
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
