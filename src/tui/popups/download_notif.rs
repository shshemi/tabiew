use ratatui::widgets::{Gauge, Widget};

use crate::{
    misc::{config::theme, download::BackgroundDownloader},
    tui::{component::Component, widgets::block::Block},
};

pub struct DownloadNotification {
    dl: BackgroundDownloader,
    title: String,
}

impl DownloadNotification {
    pub fn new(title: String, dl: BackgroundDownloader) -> Self {
        DownloadNotification { title, dl }
    }
}

impl Component for DownloadNotification {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: crate::tui::component::FocusState,
    ) {
        let mut gauge = Gauge::default()
            .block(Block::default().title(self.title.as_str()).into_widget())
            .gauge_style(theme().block());
        if let Some(ratio) = self.dl.info().ratio() {
            gauge = gauge.ratio(ratio)
        }
        gauge.render(area, buf);
    }
}
