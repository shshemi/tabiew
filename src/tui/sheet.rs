use std::iter;

use crossterm::event::{KeyCode, KeyModifiers};
use indexmap::IndexMap;
use itertools::chain;
use polars::{
    datatypes::PlSmallStr,
    prelude::{AnyValue, DataType},
};
use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    handler::message::Message,
    misc::{
        buffer_ext::BufferExt, config::theme, osc52::CopyToClipboardOsc52, polars_ext::AnyValueExt,
    },
    tui::{
        app_default::AppDefault,
        component::Component,
        tag_line::{Tag, TagLine},
        utils::Scroll,
    },
};

#[derive(Debug)]
pub struct Sheet {
    scroll: Scroll,
    row: usize,
    values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>,
}

impl Sheet {
    pub fn new(row: usize, values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) -> Self {
        Self {
            scroll: Default::default(),
            row,
            values,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll.up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll.down();
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn set(&mut self, row: usize, values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) {
        self.row = row;
        self.values = values;
    }
}

impl Component for Sheet {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        buf.clear(area);

        let pg = paragraph(&self.values).block(
            Block::app_default()
                .title(format!("Row {}", self.row + 1))
                .title_bottom(
                    TagLine::new()
                        .mono_color()
                        .centered()
                        .tag(Tag::new(" Scroll ", " Shift + J / K "))
                        .tag(Tag::new(" Copy ", " C ")),
                )
                .title_alignment(Alignment::Center),
        );

        self.scroll
            .adjust(pg.line_count(area.width), area.height.saturating_sub(2));

        pg.scroll((self.scroll.val_u16(), 0)).render(area, buf);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Char('K'), KeyModifiers::NONE)
            | (KeyCode::Char('K'), KeyModifiers::SHIFT)
            | (KeyCode::Up, KeyModifiers::SHIFT) => {
                self.scroll.up();
                true
            }
            (KeyCode::Char('J'), KeyModifiers::NONE)
            | (KeyCode::Char('J'), KeyModifiers::SHIFT)
            | (KeyCode::Down, KeyModifiers::SHIFT) => {
                self.scroll.down();
                true
            }
            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                let text = self
                    .values
                    .iter()
                    .map(|(name, (value, _))| format!("{}\n{}", name, value.to_multi_line()))
                    .collect::<Vec<_>>()
                    .join("\n\n");
                text.copy_to_clipboard_via_osc52();
                Message::AppShowToast(format!("Row #{} copied to clipboard", self.row + 1))
                    .enqueue();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::PaneDismissSheet.enqueue();
                true
            }

            _ => false,
        }
    }
}

fn section_header(idx: usize, name: &str, dtype: &DataType) -> Line<'static> {
    Line::from(vec![
        Span::raw(name.to_owned()).style(theme().header(idx)),
        Span::raw(format!(" ({dtype})")).style(theme().header(idx).remove_modifier(Modifier::BOLD)),
    ])
}

fn section_content(value: &AnyValue<'static>) -> Vec<Line<'static>> {
    match value {
        AnyValue::Null => {
            vec![Line::raw("null").style(theme().subtext().add_modifier(Modifier::ITALIC))]
        }
        value => value
            .to_multi_line()
            .lines()
            .map(|line| Line::raw(line.to_owned()).style(theme().text()))
            .collect(),
    }
}

fn paragraph(values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) -> Paragraph<'static> {
    Paragraph::new(
        values
            .iter()
            .enumerate()
            .flat_map(|(idx, (name, (value, dtype)))| {
                chain!(
                    iter::once(section_header(idx, name, dtype)),
                    section_content(value),
                    iter::once(Line::raw("\n"))
                )
            })
            .collect::<Vec<_>>(),
    )
    .style(theme().text())
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
}
