
use itertools::{izip, Itertools};
use ratatui::{prelude::*, widgets::*};

use crate::{
    app::{StatusBar, Tabular},
    command_pallete::CommandPallete,
    theme::Styler,
    utils::{line_count, ValuePool2D},
};

/// Renders the user interface widgets.
pub fn render<Theme: Styler>(
    tabular: &mut Tabular,
    status_bar: &mut StatusBar,
    frame: &mut Frame,
) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());

    // Draw table / item
    if let Some(scroll) = &mut tabular.detailed_view {
        // Set visible rows = 0
        tabular.rendered_rows = 0;
        let space = layout[0].inner(Margin::new(1, 1));
        let title = format!(" {} ", tabular.select + 1);

        let values = tabular.value_pool.get_row(tabular.select);

        let (paragraph, line_count) =
            paragraph_from_headers_values::<Theme>(&title, &tabular.headers, &values, space.width);

        scroll.adjust(line_count, space.height as usize);
        frame.render_widget(paragraph.scroll(((*scroll).into(), 0)), layout[0]);
    } else {
        // Set visible rows = table height - 1 (if header)
        tabular.rendered_rows = layout[0].height.saturating_sub(1);
        tabular.adjust_offset();

        let mut local_st = TableState::new()
            .with_offset(0)
            .with_selected(tabular.select.saturating_sub(tabular.offset));

        frame.render_stateful_widget(
            tabulate::<Theme>(
                &tabular.value_pool,
                &tabular.widths,
                &tabular.headers,
                tabular.offset,
                tabular.rendered_rows as usize,
            ),
            layout[0],
            &mut local_st,
        );


    }

    match &mut status_bar.state {
        crate::app::StatusBarState::Normal => frame.render_widget(
            Line::default()
                .spans([
                    Span::raw(format!(
                        "Row: {:<width$} ",
                        tabular.select + 1,
                        width = tabular.value_pool.height().to_string().len()
                    )),
                    Span::raw(format!(
                        "Table Size: {} x {} ",
                        tabular.value_pool.height(),
                        tabular.value_pool.width()
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
    values: &'a [&str],
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
    value_pool: &'a ValuePool2D,
    widths: &'a [usize],
    headers: &'a [String],
    offset: usize,
    length: usize,
) -> Table<'a> {
    Table::new(
        (offset..offset + length)
            .map(|row_idx| {
                Row::new(value_pool.get_row(row_idx).into_iter().map(Cell::new))
                    .style(Theme::table_row(row_idx))
            })
            .collect_vec(),
        widths
            .iter()
            .copied()
            .map(|w| Constraint::Length(w as u16))
            .collect::<Vec<_>>(),
    )
    .header(header_row::<Theme>(headers))
    .highlight_style(Theme::table_highlight())
}

fn header_row<Theme: Styler>(df: &[String]) -> Row {
    Row::new(
        df.iter()
            .enumerate()
            .map(|(col_idx, name)| {
                Cell::new(name.as_str()).style(Theme::table_header_cell(col_idx))
            })
            .collect::<Vec<_>>(),
    )
    .style(Theme::table_header())
}

fn invert_style(mut style: Style) -> Style {
    std::mem::swap(&mut style.bg, &mut style.fg);
    style
}
