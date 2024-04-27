use polars::{datatypes::AnyValue, frame::DataFrame};
use ratatui::{prelude::*, widgets::*};

use crate::{
    app::App,
    theme::{Styler, Theme},
    utils::zip_iters,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());
    app.table_height = layout[0].height - 1;
    let table_slice = &app
        .data_frame
        .slice(app.table_offset.0, app.table_height as usize);
    frame.render_stateful_widget(
        Table::new(
            rows_from_dataframe(table_slice),
            widths_from_dataframe(table_slice),
        )
        .header(header_from_dataframe(table_slice))
        .highlight_style(Style::new().bg(Color::Yellow)),
        layout[0],
        &mut TableState::new(),
    );
    frame.render_widget(Line::default().style(Theme::status_bar()), layout[1]);
}

fn cell_from_value(value: AnyValue) -> Cell {
    Cell::new(match value {
        AnyValue::Null => "".to_owned(),
        AnyValue::Boolean(v) => format!("{}", v),
        AnyValue::String(v) => v.to_string(),
        AnyValue::UInt8(v) => format!("{}", v),
        AnyValue::UInt16(v) => format!("{}", v),
        AnyValue::UInt32(v) => format!("{}", v),
        AnyValue::UInt64(v) => format!("{}", v),
        AnyValue::Int8(v) => format!("{}", v),
        AnyValue::Int16(v) => format!("{}", v),
        AnyValue::Int32(v) => format!("{}", v),
        AnyValue::Int64(v) => format!("{}", v),
        AnyValue::Float32(v) => format!("{}", v),
        AnyValue::Float64(v) => format!("{}", v),
        AnyValue::Date(v) => format!("{}", v),
        AnyValue::Datetime(v1, v2, v3) => format!("{} {} {:?}", v1, v2, v3),
        AnyValue::Duration(v1, v2) => format!("{} {}", v1, v2),
        AnyValue::Time(v) => format!("{}", v),
        AnyValue::List(v) => format!("{}", v),
        AnyValue::StringOwned(v) => format!("{}", v),
        AnyValue::Binary(v) => format!("{:?}", v),
        AnyValue::BinaryOwned(v) => format!("{:?}", v),
        AnyValue::Decimal(v1, v2) => format!("{}.{}", v1, v2),
    })
}

fn widths_from_dataframe(df: &DataFrame) -> Vec<Constraint> {
    df.get_column_names()
        .into_iter()
        .map(|col| Constraint::Min(col.len() as u16))
        .collect::<Vec<_>>()
}

fn rows_from_dataframe(df: &DataFrame) -> Vec<Row> {
    zip_iters(df.iter().map(|series| series.iter()))
        .enumerate()
        .map(|(row_idx, row)| {
            Row::new(
                row.into_iter()
                    .enumerate()
                    .map(|(col_idx, value)| {
                        cell_from_value(value).style(Theme::table_cell(row_idx, col_idx))
                    })
                    .collect::<Vec<_>>(),
            )
            .style(Theme::table_row(row_idx))
        })
        .collect::<Vec<_>>()
}

fn header_from_dataframe(df: &DataFrame) -> Row {
    Row::new(
        df.get_column_names()
            .into_iter()
            .enumerate()
            .map(|(col_idx, name)| Cell::new(name).style(Theme::table_header_cell(col_idx)))
            .collect::<Vec<_>>(),
    )
    .style(Theme::table_header())
}
