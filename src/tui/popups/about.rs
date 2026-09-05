use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    handler::message::Message,
    misc::{buffer_ext::BufferExt, config::theme},
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
        widgets::tabiew_logo::TabiewLogo,
    },
};

const PADDING_X: u16 = 2;
const PADDING_Y: u16 = 1;
const FRAME: u16 = 2;
const GAP: u16 = 1;
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

#[derive(Debug, Default)]
pub struct About;

impl Component for About {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let area = buf.area;
        let label_width = entries()
            .iter()
            .map(|(label, _)| label.width())
            .max()
            .unwrap_or(0);
        let chrome_x = FRAME + PADDING_X * 2;
        let chrome_y = FRAME + PADDING_Y * 2;

        let line_width = |value: &str| label_width + 2 + value.width();
        let room = area.width.saturating_sub(chrome_x) as usize;
        let entries = entries()
            .into_iter()
            .filter(|(_, value)| line_width(value) <= room)
            .collect::<Vec<_>>();
        let info_width = entries
            .iter()
            .map(|(_, value)| line_width(value))
            .max()
            .unwrap_or(0) as u16;
        let info_height = entries.len() as u16;
        let gap = if entries.is_empty() { 0 } else { GAP };

        let available = Rect {
            width: area.width.saturating_sub(chrome_x),
            height: area.height.saturating_sub(chrome_y + info_height + gap),
            ..area
        };
        let Some((logo_width, logo_height)) = TabiewLogo::size(available) else {
            return;
        };
        let content_width = logo_width.max(info_width);
        let content_height = logo_height + gap + info_height;
        let width = content_width + chrome_x;
        let height = content_height + chrome_y;
        if width > area.width || height > area.height {
            return;
        }

        let area = Rect {
            x: area.x + (area.width - width) / 2,
            y: area.y + (area.height - height) / 2,
            width,
            height,
        };
        buf.clear(area);
        let block = Block::app_default().app_title(icons::INFO.title("About"));
        let inner = block.inner(area);
        block.render(area, buf);

        let content = Rect {
            x: inner.x + PADDING_X,
            y: inner.y + PADDING_Y,
            width: content_width,
            height: content_height,
        };
        TabiewLogo.render(
            Rect {
                height: logo_height,
                ..content
            },
            buf,
        );
        Paragraph::new(
            entries
                .into_iter()
                .map(|(label, value)| {
                    Line::from(vec![
                        Span::styled(format!("{label:<label_width$}  "), theme().subtext()),
                        Span::styled(value, theme().text()),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .style(theme().text())
        .render(
            Rect {
                x: content.x + (content_width - info_width) / 2,
                y: content.y + logo_height + gap,
                width: info_width,
                height: info_height,
            },
            buf,
        );
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::AppDismissOverlay.enqueue();
                true
            }
            _ => false,
        }
    }
}

fn entries() -> Vec<(String, String)> {
    vec![
        (
            icons::TAG.title("Version"),
            env!("CARGO_PKG_VERSION").to_owned(),
        ),
        (icons::GITHUB.title("GitHub"), REPOSITORY.to_owned()),
        (icons::BOOK.title("Wiki"), format!("{REPOSITORY}/wiki")),
    ]
}
