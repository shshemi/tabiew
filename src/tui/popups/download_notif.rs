use ratatui::widgets::{Gauge, Widget};

use crate::{
    misc::{config::theme, download::BackgroundDownloader},
    tui::{component::Component, widgets::block::Block},
};

pub struct DownloadNotification {
    dl: BackgroundDownloader,
    title: String,
}

impl Component for DownloadNotification {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: crate::tui::component::FocusState,
    ) {
        Gauge::default()
            .block(Block::default().title(self.title.as_str()).into_widget())
            .gauge_style(theme().text())
            .ratio(self.dl.info().ratio().unwrap_or(1.0))
            .render(area, buf);
    }
}
