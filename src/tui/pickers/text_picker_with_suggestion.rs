use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    widgets::{Borders, Clear, List, ListState, StatefulWidget, Widget},
};

use crate::{
    handler::message::Message,
    misc::globals::theme,
    tui::{
        component::Component,
        widgets::{block::Block, input::Input},
    },
};

pub struct TextPickerWithSuggestion<P> {
    title: String,
    input: Input,
    list_state: ListState,
    cached_query: String,
    cached_items: Vec<String>,
    provider: P,
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
        let list = List::default()
            .items(self.cached_items.iter().map(String::as_str))
            .highlight_style(theme().row_highlighted())
            .block(
                Block::default()
                    .border_set(Set {
                        top_left: VERTICAL_RIGHT,
                        top_right: VERTICAL_LEFT,
                        ..ROUNDED
                    })
                    .into_widget(),
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
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);
    }
    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if self.input.handle(event) {
            if self.cached_query != self.input.value() {
                self.cached_query.clear();
                self.cached_query.push_str(self.input.value());
                self.cached_items.clear();
                self.cached_items
                    .extend(self.provider.suggestions(self.input.value()));
            }
            true
        } else {
            match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                _ => false,
            }
        }
    }
}

pub trait Provider {
    fn suggestions(&self, query: &str) -> impl Iterator<Item = String>;
}
