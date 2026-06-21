use std::default::Default;
use std::{borrow::Cow, ops::Add};

use ratatui::{
    layout::{Constraint, Layout},
    style::Modifier,
    text::{Line, Span},
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    misc::config::theme,
    tui::{Pane, pane::TableDescription},
};

pub struct StatusBar<'a> {
    pane: &'a Pane,
    sel_tab: usize,
    tot_tab: usize,
}

impl<'a> StatusBar<'a> {
    pub fn new(pane: &'a Pane, selected_tab: usize, total_tabs: usize) -> Self {
        Self {
            pane,
            sel_tab: selected_tab,
            tot_tab: total_tabs,
        }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let tab_tag = Tag::new(
            "Tab",
            format!(
                "{:>width$} / {}",
                self.sel_tab.add(1),
                self.tot_tab,
                width = self.tot_tab.to_string().len()
            ),
            1,
        );
        let row_tag = Tag::new(
            "Row",
            format!(
                "{:>width$}",
                self.pane.table().selected().unwrap_or_default().add(1),
                width = self.pane.table().data_frame().height().to_string().len(),
            ),
            2,
        );
        let shp_tag = Tag::new(
            "Shape",
            format!(
                "{} x {}",
                self.pane.table().data_frame().height(),
                self.pane.table().data_frame().width()
            ),
            3,
        );
        let [history_area, tab_area, row_area, shp_area] = Layout::horizontal([
            Constraint::Fill(3),
            Constraint::Length(tab_tag.width()),
            Constraint::Length(row_tag.width()),
            Constraint::Length(shp_tag.width()),
        ])
        .spacing(1)
        .areas(area);

        tab_tag.line().render(tab_area, buf);
        row_tag.line().render(row_area, buf);
        shp_tag.line().render(shp_area, buf);

        if let Some(history) = History::new(self.pane.iter_descriptions()) {
            history
                .fitted(history_area.width)
                .line()
                .render(history_area, buf);
        }
    }
}

const SEPARATOR: &str = "\u{25B6}";
const SUMMARIZED: &str = " ... ";

struct History<'a> {
    first: FirstHistoryItem<'a>,
    nexts: Vec<NextHistoryItem<'a>>,
    shrinked: bool,
}

impl<'a> History<'a> {
    fn new(t: impl IntoIterator<Item = &'a TableDescription>) -> Option<Self> {
        let mut iter = t.into_iter();
        let first = iter.next()?;
        Some(Self {
            first: FirstHistoryItem::new(first),
            nexts: iter.map(NextHistoryItem::new).collect(),
            shrinked: false,
        })
    }

    fn required_width(&self) -> u16 {
        let first = self.first.width();
        let nexts = self
            .nexts
            .iter()
            .map(|n| n.width() + SEPARATOR.width() as u16)
            .sum::<u16>();

        if self.shrinked {
            first + nexts + (SUMMARIZED.width() + SEPARATOR.width()) as u16
        } else {
            first + nexts
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
        let mut spans = self.first.spans().collect::<Vec<_>>();
        if self.shrinked {
            spans.extend([
                Span::styled(SEPARATOR, theme().tag(0).reversed()),
                Span::styled(SUMMARIZED, theme().tag(0).reversed()),
            ]);
        }
        for item in self.nexts {
            spans.push(Span::styled(SEPARATOR, theme().tag(0).reversed()));
            spans.extend(item.spans());
        }

        Line::default().spans(spans)
    }
}

struct FirstHistoryItem<'a> {
    td: &'a TableDescription,
}

impl<'a> FirstHistoryItem<'a> {
    fn new(td: &'a TableDescription) -> Self {
        Self { td }
    }

    fn width(&self) -> u16 {
        (4 + self.td.variant().width() + self.td.description().width()) as u16
    }

    fn spans(&self) -> impl Iterator<Item = Span<'a>> {
        [
            Span::styled(" ", theme().tag(0)),
            Span::styled(self.td.variant(), theme().tag(0)),
            Span::styled(" ", theme().tag(0)),
            Span::styled(" ", theme().tag(0).add_modifier(Modifier::REVERSED)),
            Span::styled(
                self.td.description().trim(),
                theme().tag(0).add_modifier(Modifier::REVERSED),
            ),
            Span::styled(" ", theme().tag(0).add_modifier(Modifier::REVERSED)),
        ]
        .into_iter()
    }
}

struct NextHistoryItem<'a> {
    td: &'a TableDescription,
}

impl<'a> NextHistoryItem<'a> {
    fn new(td: &'a TableDescription) -> Self {
        Self { td }
    }

    fn width(&self) -> u16 {
        (4 + self.td.variant().width() + self.td.description().width()) as u16
    }

    fn spans(&self) -> impl Iterator<Item = Span<'a>> {
        [
            Span::styled(" ", theme().tag(0).reversed()),
            Span::styled(self.td.variant(), theme().tag(0).reversed()),
            Span::styled("[", theme().tag(0).reversed()),
            Span::styled(self.td.description().trim(), theme().tag(0).reversed()),
            Span::styled("] ", theme().tag(0).reversed()),
        ]
        .into_iter()
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
