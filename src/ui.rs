use itertools::{izip, Itertools};
use ratatui::{prelude::*, widgets::*};

use crate::{
    app::{StatusBar, Table},
    command_pallete::CommandPallete,
    theme::Styler,
    utils::{any_value_into_string, line_count, zip_iters},
};

/// Renders the user interface widgets.
pub fn render<Theme: Styler>(tabular: &mut Table, status_bar: &mut StatusBar, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());

    // Draw table / item
    if let Some(scroll) = &mut tabular.detailed_view {
        // Set visible rows = 0
        tabular.rendered_rows = 0;
        let space = layout[0].inner(Margin::new(1, 1));
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
            .map(any_value_into_string)
            .collect_vec();

        let (paragraph, line_count) =
            paragraph_from_headers_values::<Theme>(&title, &headers, &values, space.width);

        scroll.adjust(line_count, space.height as usize);
        frame.render_widget(paragraph.scroll(((*scroll).into(), 0)), layout[0]);
    } else {
        // Set visible rows = table height - 1 (if header)
        tabular.rendered_rows = layout[0].height.saturating_sub(1);
        tabular.adjust_offset();

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

        let local_tbl = tabulate::<Theme>(&local_df, &local_widths, tabular.offset);

        let mut local_st = TableState::new()
            .with_offset(0)
            .with_selected(tabular.select.saturating_sub(tabular.offset));
        frame.render_stateful_widget(local_tbl, layout[0], &mut local_st);
    }

    match &mut status_bar.state {
        crate::app::StatusBarState::Normal => frame.render_widget(
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

        crate::app::StatusBarState::Error(msg) => frame.render_widget(
            Line::raw(msg.as_str())
                .alignment(Alignment::Center)
                .style(Theme::status_bar_red()),
            layout[1],
        ),

        crate::app::StatusBarState::Command(text) => {
            frame.render_stateful_widget(
                CommandPallete::new(
                    Theme::status_bar_green(),
                    invert_style(Theme::status_bar_green()),
                ),
                layout[1],
                text,
            );
        }
    }
}

fn paragraph_from_headers_values<'a, Theme: Styler>(
    title: &'a str,
    headers: &'a [String],
    values: &'a [String],
    width: u16,
) -> (Paragraph<'a>, usize) {
    let lines = izip!(headers, values.iter())
        .enumerate()
        .flat_map(|(idx, (header, value))| lines_from_header_value::<Theme>(idx, header, value))
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

fn lines_from_header_value<'a, Theme: Styler>(
    idx: usize,
    header: &'a str,
    value: &'a str,
) -> Vec<Line<'a>> {
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

pub fn tabulate<'a, Theme: Styler>(
    data_frame: &'a polars::frame::DataFrame,
    width: &'a [Constraint],
    offset: usize,
) -> ratatui::widgets::Table<'a> {
    ratatui::widgets::Table::new(rows_from_dataframe::<Theme>(data_frame, offset), width)
        .header(header_from_dataframe::<Theme>(data_frame))
        .highlight_style(Theme::table_highlight())
}

fn rows_from_dataframe<Theme: Styler>(df: &polars::frame::DataFrame, offset: usize) -> Vec<Row> {
    zip_iters(df.iter().map(|series| series.iter()))
        .enumerate()
        .map(|(row_idx, row)| {
            Row::new(
                row.into_iter()
                    .enumerate()
                    .map(|(col_idx, value)| {
                        Cell::new(any_value_into_string(value))
                            .style(Theme::table_cell(row_idx, col_idx))
                    })
                    .collect::<Vec<_>>(),
            )
            .style(Theme::table_row(offset + row_idx))
        })
        .collect::<Vec<_>>()
}

fn header_from_dataframe<Theme: Styler>(df: &polars::frame::DataFrame) -> Row {
    Row::new(
        df.get_column_names()
            .into_iter()
            .enumerate()
            .map(|(col_idx, name)| Cell::new(name).style(Theme::table_header_cell(col_idx)))
            .collect::<Vec<_>>(),
    )
    .style(Theme::table_header())
}

fn invert_style(mut style: Style) -> Style {
    std::mem::swap(&mut style.bg, &mut style.fg);
    style
}
