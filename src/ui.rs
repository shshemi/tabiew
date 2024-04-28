use ratatui::{prelude::*, widgets::*};

use crate::{
    app::App,
    theme::{Styler, Theme},
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());
    
    frame.render_stateful_widget(&app.table, layout[0], &mut app.table_state);
    frame.render_widget(Line::default().style(Theme::status_bar()), layout[1]);
}

