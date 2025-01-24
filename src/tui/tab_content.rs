use std::marker::PhantomData;

use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    text::Line,
    widgets::{Block, BorderType, StatefulWidget},
};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    search_bar::{SearchBar, SearchBarState},
    sheet::{Sheet, SheetState},
    status_bar::{NewStatusBar, NewStatusBarTag},
};
use crate::{search::Search, tui::theme::Styler, utils::polars_ext::GetSheetSections};

#[derive(Debug)]
pub enum Modal {
    Sheet(SheetState),
    Search(Search, SearchBarState),
}

#[derive(Debug)]
pub enum Source {
    Help,
    Schema,
    Name(String),
    Query(String),
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        match self {
            Source::Help => "Help",
            Source::Schema => "Schema",
            Source::Name(name) => name.as_str(),
            Source::Query(query) => query.as_str(),
        }
    }
}

#[derive(Debug)]
pub struct TabContentState {
    table_state: DataFrameTableState,
    modal: Option<Modal>,
    tabular_source: Source,
    original_frame: DataFrame,
}

impl TabContentState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_source: Source) -> Self {
        Self {
            table_state: DataFrameTableState::new(data_frame.clone()),
            modal: None,
            tabular_source,
            original_frame: data_frame,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Some(Modal::Search(search, _)) = &mut self.modal {
            if let Some(df) = search.latest() {
                self.table_state.set_data_frame(df);
            }
        }
    }

    pub fn select_up(&mut self, len: usize) {
        self.table_state.select_up(len);
    }

    pub fn select_down(&mut self, len: usize) {
        self.table_state.select_down(len);
    }

    pub fn select_first(&mut self) {
        self.table_state.select_first();
    }

    pub fn select_last(&mut self) {
        self.table_state.select_last();
    }

    pub fn select_random(&mut self) {
        let mut rng = rand::thread_rng();
        self.table_state
            .select(rng.gen_range(0..self.table_state.height()));
    }

    pub fn select(&mut self, select: usize) {
        self.table_state.select(select);
    }

    pub fn selected(&self) -> usize {
        self.table_state.selected()
    }

    pub fn sheet_scroll_up(&mut self) {
        if let Some(Modal::Sheet(scroll)) = &mut self.modal {
            scroll.scroll_up();
        }
    }

    pub fn sheet_scroll_down(&mut self) {
        if let Some(Modal::Sheet(scroll)) = &mut self.modal {
            scroll.scroll_down();
        }
    }

    pub fn table_scroll_left(&mut self) {
        self.table_state.scroll_left();
    }

    pub fn table_scroll_right(&mut self) {
        self.table_state.scroll_right();
    }

    pub fn table_goto_start(&mut self) {
        self.table_state.scroll_start();
    }

    pub fn table_goto_end(&mut self) {
        self.table_state.scroll_end();
    }

    pub fn expanded(&self) -> bool {
        if self.modal.is_none() {
            self.table_state.expanded()
        } else {
            false
        }
    }

    pub fn toggle_expansion(&mut self) {
        self.table_state.toggle_expansion();
    }

    pub fn page_len(&self) -> usize {
        self.table_state.rendered_rows().into()
    }

    pub fn switch_view(&mut self) {
        match self.modal {
            None => self.sheet_mode(),
            Some(Modal::Sheet(_)) => self.table_mode(),
            _ => (),
        }
    }

    pub fn table_mode(&mut self) {
        self.modal = None;
    }

    pub fn sheet_mode(&mut self) {
        self.modal = Some(Modal::Sheet(Default::default()));
    }

    pub fn search_mode(&mut self) {
        match &self.modal {
            None => {
                self.modal = Some(Modal::Search(
                    Search::new(self.original_frame.clone(), Default::default()),
                    SearchBarState::default(),
                ));
            }
            _ => (),
        }
    }

    pub fn search_commit(&mut self) {
        match &self.modal {
            Some(Modal::Search(search, _)) => {
                if let Some(df) = search.latest() {
                    self.set_data_frame(df);
                }
            }
            _ => (),
        }
    }

    pub fn search_delete_prev(&mut self) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().delete_prev();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_delete_next(&mut self) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().delete_next();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_prev(&mut self) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().goto_prev();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_next(&mut self) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().goto_next();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_start(&mut self) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().goto_start();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_end(&mut self) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().goto_end();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_insert(&mut self, c: char) {
        if let Some(Modal::Search(search, search_bar_state)) = &mut self.modal {
            search_bar_state.input().insert(c);
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn data_frame(&self) -> &DataFrame {
        self.table_state.data_frame()
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        self.table_state.data_frame_mut()
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.table_state.set_data_frame(data_frame);
    }

    pub fn rollback(&mut self) {
        self.table_state.set_data_frame(self.original_frame.clone());
    }

    pub fn modal(&self) -> Option<&Modal> {
        self.modal.as_ref()
    }

    pub fn tabular_source(&self) -> &Source {
        &self.tabular_source
    }
}

pub struct TabContent<Theme> {
    status_bar: NewStatusBar<Theme>,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> TabContent<Theme> {
    pub fn new() -> Self {
        Self {
            status_bar: NewStatusBar::<Theme>::new(),
            _theme: Default::default(),
        }
    }

    pub fn with_tag(mut self, tag: NewStatusBarTag<Theme>) -> Self {
        self.status_bar = self.status_bar.with_tag(tag);
        self
    }
}

impl<Theme: Styler> Default for TabContent<Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> StatefulWidget for TabContent<Theme> {
    type State = TabContentState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (search_bar_area, table_area) = match state.modal {
            Some(Modal::Search(_, _)) => {
                let [a0, a1] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                (a0, a1)
            }
            _ => (Rect::default(), area),
        };
        DataFrameTable::<Theme>::new()
            .with_block(
                Block::bordered()
                    .border_style(Theme::sheet_block())
                    .border_type(BorderType::Rounded)
                    .title_top(Line::from(format!(" {} ", state.tabular_source.as_ref())))
                    .title_bottom(self.status_bar.with_tags([
                        NewStatusBarTag::new(
                            "Expended",
                            if state.table_state.expanded() {
                                "Yes"
                            } else {
                                " No"
                            },
                        ),
                        NewStatusBarTag::new(
                            "Row",
                            format!(
                                "{:>width$}",
                                state.table_state.selected(),
                                width = state.table_state.height().to_string().len()
                            ),
                        ),
                        NewStatusBarTag::new(
                            "Shape",
                            format!(
                                "{} x {}",
                                state.table_state.height(),
                                state.table_state.data_frame().width()
                            ),
                        ),
                    ])),
            )
            .render(table_area, buf, &mut state.table_state);

        match &mut state.modal {
            Some(Modal::Sheet(sheet_state)) => {
                let area = area.inner(Margin::new(13, 3));
                let sections = state
                    .table_state
                    .data_frame()
                    .get_sheet_sections(state.table_state.selected());
                Sheet::<Theme>::new()
                    .with_sections(sections)
                    .render(area, buf, sheet_state);
            }

            Some(Modal::Search(_, search_bar_state)) => {
                SearchBar::<Theme>::new().render(search_bar_area, buf, search_bar_state);
            }

            _ => (),
        }
    }
}
