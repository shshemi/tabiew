use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    widgets::{Borders, Clear, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::{
    misc::config::theme,
    tui::{
        component::Component,
        widgets::{block::Block, input::Input},
    },
};

#[derive(Debug, Default)]
pub struct TextPickerWithSuggestion<P: Provider> {
    title: String,
    input: Input,
    list: ListState,
    args: (String, usize),
    items: Vec<P::Suggestion>,
    provider: P,
}

impl<P> TextPickerWithSuggestion<P>
where
    P: Provider,
{
    pub fn new(title: impl Into<String>, provider: P) -> Self {
        Self {
            title: title.into(),
            input: Default::default(),
            list: ListState::default().with_selected(0.into()),
            args: (String::default(), 0),
            items: provider.suggestions("", 0),
            provider,
        }
    }

    pub fn apply_selected(&mut self) {
        if let Some(suggestion) = self.list.selected().and_then(|idx| self.items.get(idx)) {
            suggestion.apply_to(&mut self.input);
        }
    }
}

impl<P> Component for TextPickerWithSuggestion<P>
where
    P: Provider,
{
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        if self.args.0 != self.input.value() || self.args.1 != self.input.cursor() {
            self.args = (self.input.value().to_owned(), self.input.cursor());
            self.items = self
                .provider
                .suggestions(self.input.value(), self.input.cursor());
        }

        let list = List::default()
            .style(theme().text())
            .highlight_style(theme().row_highlighted())
            .block(
                Block::default()
                    .border_set(Set {
                        top_left: VERTICAL_RIGHT,
                        top_right: VERTICAL_LEFT,
                        ..ROUNDED
                    })
                    .into_widget(),
            )
            .items(
                self.items
                    .iter()
                    .map(|suggestion| ListItem::new(suggestion.title())),
            );

        let width = 80;
        let height = list.len().saturating_add(4).min(25) as u16;

        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(height)]).areas(area);

        Widget::render(Clear, area, buf);
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area);

        let input_area = {
            let block = Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .title(self.title.as_str());
            let input_inner = block.inner(input_area);
            Widget::render(block, area, buf);
            input_inner
        };
        self.input.render(input_area, buf, focus_state);
        StatefulWidget::render(list, list_area, buf, &mut self.list);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.input.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    if self.list.selected() != Some(0) {
                        self.list.select_previous();
                    } else {
                        self.list.select(Some(self.items.len().saturating_sub(1)));
                    }
                    true
                }
                (KeyCode::Down, KeyModifiers::NONE)
                | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                    if self.list.selected() != Some(self.items.len().saturating_sub(1)) {
                        self.list.select_next();
                    } else {
                        self.list.select_first();
                    }
                    true
                }
                _ => false,
            }
    }
}

pub trait Suggestion {
    fn title(&self) -> &str;
    fn apply_to(&self, input: &mut Input);
}

pub trait Provider {
    type Suggestion: Suggestion;
    fn suggestions(&self, query: &str, cursor: usize) -> Vec<Self::Suggestion>;
}
