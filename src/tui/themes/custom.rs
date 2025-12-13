use std::fs;

use anyhow::anyhow;
use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};

use crate::{AppResult, misc::paths::theme_path, tui::themes::styler::Styler};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(default)]
pub struct Custom {
    table_header: Style,
    table_headers: Vec<Style>,
    rows: Vec<Style>,
    row_highlight: Style,
    table_tags: Vec<Style>,
    block: Style,
    block_tag: Style,
    text: Style,
    text_highlighted: Style,
    subtext: Style,
    error: Style,
    gutter: Style,
    chart: Vec<Style>,
}

impl Custom {
    pub fn read_from_config_dir() -> AppResult<Self> {
        let path = theme_path().ok_or(anyhow!("Home dir not found"))?;
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}

impl Default for Custom {
    fn default() -> Self {
        Custom {
            table_header: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffffff)),
            table_headers: vec![
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ffff00)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ff00ff)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x0000ffff)),
            ],
            rows: vec![
                Style::default()
                    .bg(Color::from_u32(0x00383838))
                    .fg(Color::from_u32(0x00ffffff)),
                Style::default()
                    .bg(Color::from_u32(0x00101010))
                    .fg(Color::from_u32(0x00ffffff)),
            ],
            row_highlight: Style::default()
                .bg(Color::from_u32(0x00ffff00))
                .fg(Color::from_u32(0x00000000)),
            table_tags: vec![
                Style::default()
                    .bg(Color::from_u32(0x00ff0000))
                    .fg(Color::from_u32(0x00000000)),
                Style::default()
                    .bg(Color::from_u32(0x0000ff00))
                    .fg(Color::from_u32(0x00000000)),
            ],
            block: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffff00)),
            block_tag: Style::default()
                .bg(Color::from_u32(0x00ffff00))
                .fg(Color::from_u32(0x00000000)),
            text: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffffff)),
            subtext: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ababab)),
            error: Style::default()
                .bg(Color::from_u32(0x00ff0000))
                .fg(Color::from_u32(0x00ffffff)),
            chart: vec![
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ffff00)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ff00ff)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x0000ffff)),
            ],
            text_highlighted: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ff00ff)),
            gutter: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ff00ff)),
        }
    }
}

impl Styler for Custom {
    fn table_header(&self) -> Style {
        self.table_header
    }

    fn header(&self, idx: usize) -> Style {
        self.table_headers[idx % self.table_headers.len()]
    }

    fn row(&self, idx: usize) -> Style {
        self.rows[idx % self.rows.len()]
    }

    fn row_highlighted(&self) -> Style {
        self.row_highlight
    }

    fn tag(&self, idx: usize) -> Style {
        self.table_tags[idx % self.table_tags.len()]
    }

    fn block(&self) -> Style {
        self.block
    }

    fn block_tag(&self) -> Style {
        self.block_tag
    }

    fn text(&self) -> Style {
        self.text
    }

    fn subtext(&self) -> Style {
        self.subtext
    }

    fn error(&self) -> Style {
        self.error
    }

    fn graph(&self, idx: usize) -> Style {
        self.chart[idx % self.chart.len()]
    }

    fn text_highlighted(&self) -> Style {
        self.text_highlighted
    }

    fn gutter(&self, _: usize) -> Style {
        self.gutter
    }
}
