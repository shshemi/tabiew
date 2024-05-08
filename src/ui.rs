use itertools::{izip, Itertools};
use ratatui::{prelude::*, widgets::*};

use crate::{
    app::{StatusBar, Table},
    theme::{Styler, Theme},
    utils::{line_count, string_from_any_value, tabulate},
};

/// Renders the user interface widgets.
pub fn render(tabular: &mut Table, status_bar: &mut StatusBar, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());

    // Set visible rows = table height - 1 (if header)
    tabular.rendered_rows = layout[0].height.saturating_sub(1);
    tabular.adjust_offset();

    // Draw table / item
    if let Some(scroll) = &mut tabular.detailed_view {
        let space = layout[0].inner(&Margin::new(1, 1));
        let title = format!(" {} ", tabular.select + 1);

        let headers = tabular
            .data_frame
            .get_column_names()
            .iter()
            .map(|name| {
                format!(
                    "{} ({:?})",
                    name,
                    tabular.data_frame.column(name).unwrap().dtype()
                )
            })
            .collect_vec();

        let values = tabular
            .data_frame
            .get_row(tabular.select)
            .unwrap_or_default()
            .0
            .into_iter()
            .map(string_from_any_value)
            .collect_vec();

        let (paragraph, line_count) =
            paragraph_from_headers_values(&title, &headers, &values, space.width);

        scroll.adjust(line_count, space.height as usize);
        frame.render_widget(paragraph.scroll(((*scroll).into(), 0)), layout[0]);
    } else {
        // Building local table
        let local_df = tabular
            .data_frame
            .slice(tabular.offset as i64, tabular.rendered_rows.into());

        let local_widths = tabular
            .widths
            .iter()
            .copied()
            .map(|w| Constraint::Length(w as u16))
            .collect::<Vec<_>>();

        let highlight_symbol = format!(
            "{:>width$} ",
            tabular.select + 1,
            width = tabular.data_frame.height().to_string().len()
        );

        let local_tbl = tabulate(&local_df, &local_widths, &highlight_symbol, tabular.offset);

        let mut local_st = TableState::new()
            .with_offset(0)
            .with_selected(tabular.select.saturating_sub(tabular.offset));
        frame.render_stateful_widget(local_tbl, layout[0], &mut local_st);
    }

    match status_bar {
        crate::app::StatusBar::Normal => frame.render_widget(
            Line::default()
                .spans([
                    Span::raw(format!(
                        "Row: {:<width$} ",
                        tabular.select + 1,
                        width = tabular.data_frame.height().to_string().len()
                    )),
                    Span::raw(format!(
                        "Table Size: {} x {} ",
                        tabular.data_frame.height(),
                        tabular.data_frame.width()
                    )),
                ])
                .alignment(Alignment::Right)
                .style(Theme::status_bar_blue()),
            layout[1],
        ),

        crate::app::StatusBar::Error(msg, _) => frame.render_widget(
            Line::raw(msg.as_str())
                .alignment(Alignment::Center)
                .style(Theme::status_bar_red()),
            layout[1],
        ),

        crate::app::StatusBar::Command(text) => frame.render_widget(text.widget(), layout[1]),
    }
}

fn paragraph_from_headers_values<'a>(
    title: &'a str,
    headers: &'a [String],
    values: &'a [String],
    width: u16,
) -> (Paragraph<'a>, usize) {
    let lines = izip!(headers, values.iter())
        .enumerate()
        .flat_map(|(idx, (header, value))| lines_from_header_value(idx, header, value))
        .collect_vec();
    let lc = lines
        .iter()
        .map(|line| line_count(&line.to_string(), width as usize))
        .sum();
    let prgr = Paragraph::new(lines)
        .block(Block::new().title(title).borders(Borders::ALL))
        .style(Theme::item_block())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    (prgr, lc)
}

fn lines_from_header_value<'a>(idx: usize, header: &'a str, value: &'a str) -> Vec<Line<'a>> {
    let header_line = std::iter::once(Line::from(Span::styled(
        header,
        Theme::table_header_cell(idx),
    )));
    let value_lines = value
        .lines()
        .map(|line| Line::from(Span::styled(line, Theme::table_cell(idx, 0))));
    header_line
        .chain(value_lines)
        .chain(std::iter::once(Line::default()))
        .collect_vec()
}
