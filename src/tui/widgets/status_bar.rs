use std::default::Default;
use std::{borrow::Cow, ops::Add};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    misc::config::{config, theme},
    tui::{Pane, pane::TableDescription},
};

pub struct StatusBar<'a> {
    pane: &'a Pane,
}

impl<'a> StatusBar<'a> {
    pub fn new(pane: &'a Pane) -> Self {
        Self { pane }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let row_tag = (!self.pane.table().gutter_visible()).then(|| {
            Tag::new(
                "Row",
                format!(
                    "{:>width$}",
                    self.pane.table().selected().unwrap_or_default().add(1),
                    width = self.pane.table().data_frame().height().to_string().len(),
                ),
                2,
            )
        });
        let shp_tag = Tag::new(
            "Shape",
            format!(
                "{} x {}",
                self.pane.table().data_frame().height(),
                self.pane.table().data_frame().width()
            ),
            3,
        );
        let tags = row_tag.into_iter().chain([shp_tag]).collect::<Vec<_>>();
        let areas = Layout::horizontal(
            std::iter::once(Constraint::Fill(3))
                .chain(tags.iter().map(|tag| Constraint::Length(tag.width()))),
        )
        .spacing(1)
        .split(area);

        for (tag, area) in tags.iter().zip(areas.iter().skip(1)) {
            tag.line().render(*area, buf);
        }

        if let Some(history) = History::new(self.pane.iter_descriptions()) {
            let history_area = areas[0];
            history
                .fitted(history_area.width)
                .line()
                .render(history_area, buf);
        }
    }
}

const NERD_SEPARATOR: &str = "\u{E0B0}";
const SEPARATOR: &str = "\u{25B6}";
const SUMMARIZED: &str = " ... ";

fn separator() -> &'static str {
    if config().use_nerd_font() {
        NERD_SEPARATOR
    } else {
        SEPARATOR
    }
}

struct History<'a> {
    first: HistoryItem<'a>,
    nexts: Vec<HistoryItem<'a>>,
    shrinked: bool,
}

impl<'a> History<'a> {
    fn new(t: impl IntoIterator<Item = &'a TableDescription>) -> Option<Self> {
        let mut iter = t
            .into_iter()
            .enumerate()
            .map(|(pos, td)| HistoryItem::new(td, pos));
        Some(Self {
            first: iter.next()?,
            nexts: iter.collect(),
            shrinked: false,
        })
    }

    fn required_width(&self) -> u16 {
        let separator = separator().width() as u16;
        let items = std::iter::once(&self.first)
            .chain(self.nexts.iter())
            .map(|item| item.width())
            .sum::<u16>();
        let separators = self.nexts.len() as u16 * separator;

        if self.shrinked {
            items + separators + SUMMARIZED.width() as u16 + separator
        } else {
            items + separators
        }
    }

    fn shrink(&mut self) {
        if self.can_shrink() {
            self.nexts.remove(0);
            self.shrinked = true;
        }
    }

    fn can_shrink(&self) -> bool {
        self.nexts.len() > 1
    }

    fn fitted(mut self, width: u16) -> Self {
        while width < self.required_width() && self.can_shrink() {
            self.shrink();
        }
        self
    }

    fn line(self) -> Line<'a> {
        let mut prev = self.first.tail_style();
        let mut spans = self.first.spans().collect::<Vec<_>>();

        if self.shrinked {
            let style = theme().tag(1);
            spans.push(chevron(prev, style));
            spans.push(Span::styled(SUMMARIZED, style));
            prev = style;
        }

        for item in self.nexts {
            spans.push(chevron(prev, item.head_style()));
            prev = item.tail_style();
            spans.extend(item.spans());
        }

        Line::default().spans(spans)
    }
}

fn reversed(style: Style) -> Style {
    Style::default()
        .fg(style.bg.unwrap_or(Color::Reset))
        .bg(style.fg.unwrap_or(Color::Reset))
}

fn chevron(left: Style, right: Style) -> Span<'static> {
    Span::styled(
        separator(),
        Style::default()
            .fg(left.bg.unwrap_or(Color::Reset))
            .bg(right.bg.unwrap_or(Color::Reset)),
    )
}

struct HistoryItem<'a> {
    td: &'a TableDescription,
    pos: usize,
}

impl<'a> HistoryItem<'a> {
    fn new(td: &'a TableDescription, pos: usize) -> Self {
        Self { td, pos }
    }

    fn head_style(&self) -> Style {
        theme().tag(self.pos)
    }

    fn tail_style(&self) -> Style {
        if self.td.description().trim().is_empty() {
            self.head_style()
        } else {
            reversed(self.head_style())
        }
    }

    fn width(&self) -> u16 {
        let description = self.td.description().trim();
        if description.is_empty() {
            (2 + self.td.variant().width()) as u16
        } else {
            (4 + self.td.variant().width() + description.width()) as u16
        }
    }

    fn spans(&self) -> impl Iterator<Item = Span<'a>> {
        let head = self.head_style();
        let tail = self.tail_style();
        let description = self.td.description().trim();
        [
            Some(Span::styled(" ", head)),
            Some(Span::styled(self.td.variant(), head)),
            Some(Span::styled(" ", head)),
            (!description.is_empty()).then(|| Span::styled(" ", tail)),
            (!description.is_empty()).then(|| Span::styled(description, tail)),
            (!description.is_empty()).then(|| Span::styled(" ", tail)),
        ]
        .into_iter()
        .flatten()
    }
}

struct Tag<'a> {
    key: Cow<'a, str>,
    val: Cow<'a, str>,
    pos: usize,
}

impl<'a> Tag<'a> {
    fn new(key: impl Into<Cow<'a, str>>, val: impl Into<Cow<'a, str>>, pos: usize) -> Self {
        Self {
            key: key.into(),
            val: val.into(),
            pos,
        }
    }

    fn line(&self) -> Line<'_> {
        Line::default().spans([
            Span::styled(" ", theme().tag(self.pos)),
            Span::styled(self.key.as_ref(), theme().tag(self.pos)),
            Span::styled(" ", theme().tag(self.pos)),
            Span::styled(" ", theme().tag(self.pos).add_modifier(Modifier::REVERSED)),
            Span::styled(
                self.val.as_ref(),
                theme().tag(self.pos).add_modifier(Modifier::REVERSED),
            ),
            Span::styled(" ", theme().tag(self.pos).add_modifier(Modifier::REVERSED)),
        ])
    }
    fn width(&self) -> u16 {
        (4 + self.key.width() + self.val.width()) as u16
    }
}
