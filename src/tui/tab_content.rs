use std::marker::PhantomData;

use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, BorderType, Borders, StatefulWidget},
};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    search_bar::{SearchBar, SearchBarState},
    sheet::{Sheet, SheetState},
    status_bar::{StatusBar, StatusBarTag},
};
use crate::{search::Search, tui::theme::Styler, utils::polars_ext::GetSheetSections, AppResult};

#[derive(Debug)]
pub enum Modal {
    Sheet(SheetState),
    Search(Search, SearchBarState, DataFrame),
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
    table: DataFrameTableState,
    modal: Option<Modal>,
    source: Source,
}

impl TabContentState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_source: Source) -> Self {
        Self {
            table: DataFrameTableState::new(data_frame.clone()),
            modal: None,
            source: tabular_source,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Some(Modal::Search(search, _, _)) = &mut self.modal {
            if let Some(df) = search.latest() {
                self.table.set_data_frame(df);
            }
        }
    }

    pub fn select_up(&mut self, len: usize) {
        self.table.select_up(len);
    }

    pub fn select_down(&mut self, len: usize) {
        self.table.select_down(len);
    }

    pub fn select_first(&mut self) {
        self.table.select_first();
    }

    pub fn select_last(&mut self) {
        self.table.select_last();
    }

    pub fn select_random(&mut self) {
        let mut rng = rand::rng();
        self.table.select(rng.random_range(0..self.table.height()));
    }

    pub fn select(&mut self, select: usize) {
        self.table.select(select);
    }

    pub fn selected(&self) -> usize {
        self.table.selected()
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
        self.table.scroll_left();
    }

    pub fn table_scroll_right(&mut self) {
        self.table.scroll_right();
    }

    pub fn table_goto_start(&mut self) {
        self.table.scroll_start();
    }

    pub fn table_goto_end(&mut self) {
        self.table.scroll_end();
    }

    pub fn expanded(&self) -> bool {
        if self.modal.is_none() {
            self.table.expanded()
        } else {
            false
        }
    }

    pub fn toggle_expansion(&mut self) -> AppResult<()> {
        self.table.toggle_expansion()
    }

    pub fn page_len(&self) -> usize {
        self.table.rendered_rows().into()
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
        if self.modal.is_none() {
            self.modal = Some(Modal::Search(
                Search::new(self.table.data_frame().clone(), Default::default()),
                SearchBarState::default(),
                self.table.data_frame().clone(),
            ));
        }
    }

    pub fn search_commit(&mut self) {
        if let Some(Modal::Search(search, _, _)) = &self.modal {
            if let Some(df) = search.latest() {
                self.set_data_frame(df);
            }
        }
    }

    pub fn search_delete_prev(&mut self) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().delete_prev();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_delete_next(&mut self) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().delete_next();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_prev(&mut self) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().goto_prev();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_next(&mut self) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().goto_next();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_start(&mut self) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().goto_start();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_end(&mut self) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().goto_end();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_insert(&mut self, c: char) {
        if let Some(Modal::Search(search, search_bar_state, og_frame)) = &mut self.modal {
            search_bar_state.input().insert(c);
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    og_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_rollback(&mut self) {
        if let Some(Modal::Search(_, _, og_frame)) = self.modal.take() {
            self.table.set_data_frame(og_frame);
        }
    }

    pub fn data_frame(&self) -> &DataFrame {
        self.table.data_frame()
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        self.table.data_frame_mut()
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.table.set_data_frame(data_frame);
    }

    pub fn modal(&self) -> Option<&Modal> {
        self.modal.as_ref()
    }

    pub fn tabular_source(&self) -> &Source {
        &self.source
    }
}

pub struct TabContent<Theme> {
    status_bar: StatusBar<Theme>,
    borders: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> TabContent<Theme> {
    pub fn new() -> Self {
        Self {
            status_bar: StatusBar::<Theme>::new(),
            borders: true,
            _theme: Default::default(),
        }
    }

    pub fn with_tag(mut self, tag: StatusBarTag<Theme>) -> Self {
        self.status_bar = self.status_bar.with_tag(tag);
        self
    }

    pub fn with_borders(mut self, border: bool) -> Self {
        self.borders = border;
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
            Some(Modal::Search(_, _, _)) => {
                let [a0, a1] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                (a0, a1)
            }
            _ => (Rect::default(), area),
        };
        DataFrameTable::<Theme>::new()
            .with_block(
                Block::new()
                    .borders(if self.borders {
                        Borders::all()
                    } else {
                        Borders::empty()
                    })
                    .border_style(Theme::sheet_block())
                    .border_type(BorderType::Rounded)
                    .title_bottom(self.status_bar.with_tags([
                        match &state.source {
                            Source::Help => StatusBarTag::new("App", "Help"),
                            Source::Schema => StatusBarTag::new("App", "Schema"),
                            Source::Name(name) => StatusBarTag::new("Table", name.to_owned()),
                            Source::Query(query) => StatusBarTag::new("Query", query.to_owned()),
                        },
                        StatusBarTag::new(
                            "Auto-Fit",
                            if !state.table.expanded() {
                                "Yes"
                            } else {
                                " No"
                            },
                        ),
                        StatusBarTag::new(
                            "Row",
                            format!(
                                "{:>width$}",
                                state.table.selected() + 1,
                                width = state.table.height().to_string().len()
                            ),
                        ),
                        StatusBarTag::new(
                            "Shape",
                            format!(
                                "{} x {}",
                                state.table.height(),
                                state.table.data_frame().width()
                            ),
                        ),
                    ])),
            )
            .render(table_area, buf, &mut state.table);

        match &mut state.modal {
            Some(Modal::Sheet(sheet_state)) => {
                let area = area.inner(Margin::new(13, 3));
                let sections = state
                    .table
                    .data_frame()
                    .get_sheet_sections(state.table.selected());
                Sheet::<Theme>::new()
                    .with_sections(sections)
                    .render(area, buf, sheet_state);
            }

            Some(Modal::Search(_, search_bar_state, _)) => {
                SearchBar::<Theme>::new().with_selection(true).render(
                    search_bar_area,
                    buf,
                    search_bar_state,
                );
            }

            _ => (),
        }
    }
}
