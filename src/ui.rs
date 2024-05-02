use ratatui::{prelude::*, widgets::*};

use crate::{
    app::App,
    theme::{Styler, Theme},
    utils::tabulate,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());

    // Set visible rows = table height - 1 (if header)
    app.rendered_rows = layout[0].height - 1;
    app.adjust_offset();

    // Building local table
    let local_df = app
        .data_frame
        .slice(app.offset as i64, app.rendered_rows.into());

    let local_widths = app
        .widths
        .iter()
        .copied()
        .map(|w| Constraint::Length(w as u16))
        .collect::<Vec<_>>();

    let highlight_symbol = format!(
        "{:>width$} ",
        app.select,
        width = app.data_frame.height().to_string().len()
    );

    let local_tbl = tabulate(&local_df, &local_widths, &highlight_symbol);

    let mut local_st = TableState::new()
        .with_offset(0)
        .with_selected(app.select.saturating_sub(app.offset));
    frame.render_stateful_widget(local_tbl, layout[0], &mut local_st);

    match &app.status {
        crate::app::AppStatus::Normal => frame.render_widget(
            Line::default()
                .spans([
                    Span::raw(format!(
                        "Row: {:<width$} ",
                        app.select,
                        width = app.data_frame.height().to_string().len()
                    )),
                    Span::raw(format!(
                        "Table Size: {} x {} ",
                        app.data_frame.height(),
                        app.data_frame.width()
                    )),
                ])
                .alignment(Alignment::Right)
                .style(Theme::status_bar_blue()),
            layout[1],
        ),

        crate::app::AppStatus::Error(msg, _) => frame.render_widget(
            Line::raw(msg)
                .alignment(Alignment::Center)
                .style(Theme::status_bar_red()),
            layout[1],
        ),

        crate::app::AppStatus::Command(text) => frame.render_widget(text.widget(), layout[1]),
    }
}
