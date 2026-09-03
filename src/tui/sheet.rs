use std::iter;

use crossterm::event::{KeyCode, KeyModifiers};
use indexmap::IndexMap;
use itertools::chain;
use polars::{
    datatypes::PlSmallStr,
    prelude::{AnyValue, DataType},
};
use unicode_width::UnicodeWidthStr;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    handler::message::Message,
    misc::{
        buffer_ext::BufferExt, config::theme, osc52::CopyToClipboardOsc52, polars_ext::AnyValueExt,
    },
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
        tag_line::{Tag, TagLine},
        utils::Scroll,
    },
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Format {
    #[default]
    Plain,
    Json,
}

impl Format {
    fn toggled(self) -> Self {
        match self {
            Format::Plain => Format::Json,
            Format::Json => Format::Plain,
        }
    }
}

#[derive(Debug)]
pub struct Sheet {
    scroll: Scroll,
    row: usize,
    values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>,
    format: Format,
}

impl Sheet {
    pub fn new(row: usize, values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) -> Self {
        Self {
            scroll: Default::default(),
            row,
            values,
            format: Default::default(),
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll.up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll.down();
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn set(&mut self, row: usize, values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) {
        self.row = row;
        self.values = values;
    }
}

impl Component for Sheet {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        buf.clear(area);

        let pg = match self.format {
            Format::Plain => plain_paragraph(&self.values),
            Format::Json => json_paragraph(&self.values, area.width.saturating_sub(2)),
        }
        .block(
            Block::app_default()
                .app_title(format!("Row {}", self.row + 1))
                .title_bottom(
                    TagLine::new()
                        .mono_color()
                        .centered()
                        .tag(Tag::new(
                            icons::HEIGHT.str("Scroll"),
                            "Shift+\u{2193}\u{2191}/JK",
                        ))
                        .tag(Tag::new(icons::COPY.str("Copy"), "C"))
                        .tag(Tag::new(icons::JSON.str("Format"), "F")),
                )
                .title_alignment(Alignment::Center),
        );

        self.scroll
            .adjust(pg.line_count(area.width), area.height.saturating_sub(2));

        pg.scroll((self.scroll.val_u16(), 0)).render(area, buf);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Char('K'), KeyModifiers::NONE)
            | (KeyCode::Char('K'), KeyModifiers::SHIFT)
            | (KeyCode::Up, KeyModifiers::SHIFT) => {
                self.scroll.up();
                true
            }
            (KeyCode::Char('J'), KeyModifiers::NONE)
            | (KeyCode::Char('J'), KeyModifiers::SHIFT)
            | (KeyCode::Down, KeyModifiers::SHIFT) => {
                self.scroll.down();
                true
            }
            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                self.format = self.format.toggled();
                self.scroll.reset();
                true
            }
            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                let text = match self.format {
                    Format::Plain => plain_text(&self.values),
                    Format::Json => json_text(&self.values),
                };
                text.copy_to_clipboard_via_osc52();
                Message::AppShowToast(format!("Row #{} copied to clipboard", self.row + 1))
                    .enqueue();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::PaneDismissSheet.enqueue();
                true
            }

            _ => false,
        }
    }
}

fn section_header(idx: usize, name: &str, dtype: &DataType) -> Line<'static> {
    Line::from(vec![
        Span::raw(name.to_owned()).style(theme().header(idx)),
        Span::raw(format!(" ({dtype})")).style(theme().header(idx).remove_modifier(Modifier::BOLD)),
    ])
}

fn section_content(value: &AnyValue<'static>) -> Vec<Line<'static>> {
    match value {
        AnyValue::Null => {
            vec![Line::raw("null").style(theme().subtext().add_modifier(Modifier::ITALIC))]
        }
        value => value
            .to_multi_line()
            .lines()
            .map(|line| Line::raw(line.to_owned()).style(theme().text()))
            .collect(),
    }
}

fn plain_text(values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) -> String {
    values
        .iter()
        .map(|(name, (value, _))| format!("{}\n{}", name, value.to_multi_line()))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn plain_paragraph(
    values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>,
) -> Paragraph<'static> {
    Paragraph::new(
        values
            .iter()
            .enumerate()
            .flat_map(|(idx, (name, (value, dtype)))| {
                chain!(
                    iter::once(section_header(idx, name, dtype)),
                    section_content(value),
                    iter::once(Line::raw("\n"))
                )
            })
            .collect::<Vec<_>>(),
    )
    .style(theme().text())
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
}

fn json_paragraph(
    values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>,
    width: u16,
) -> Paragraph<'static> {
    Paragraph::new(json_lines(values, width as usize))
        .style(theme().text())
        .alignment(Alignment::Left)
}

const INDENT: &str = "  ";

struct JsonLine {
    field: usize,
    indent: String,
    key: Option<String>,
    body: String,
    null: bool,
}

fn json_body(values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) -> Vec<JsonLine> {
    let mut out = Vec::new();
    let last = values.len().saturating_sub(1);

    for (field, (name, (value, _))) in values.iter().enumerate() {
        Line::default().extend(iter);
        let key = serde_json::to_string(name.as_str()).unwrap_or_else(|_| format!("\"{name}\""));
        let rendered =
            serde_json::to_string_pretty(&to_json(value)).unwrap_or_else(|_| String::from("null"));
        let null = matches!(value, AnyValue::Null);
        let comma = if field == last { "" } else { "," };
        let text = rendered.lines().collect::<Vec<_>>();
        let tail = text.len().saturating_sub(1);

        for (n, line) in text.into_iter().enumerate() {
            let trimmed = line.trim_start();
            let own = &line[..line.len() - trimmed.len()];
            out.push(JsonLine {
                field,
                indent: format!("{INDENT}{own}"),
                key: (n == 0).then(|| key.clone()),
                body: format!("{trimmed}{}", if n == tail { comma } else { "" }),
                null,
            });
        }
    }
    out
}

fn json_text(values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>) -> String {
    let mut out = String::from("{\n");
    for line in json_body(values) {
        out.push_str(&line.indent);
        if let Some(key) = &line.key {
            out.push_str(key);
            out.push_str(": ");
        }
        out.push_str(&line.body);
        out.push('\n');
    }
    out.push('}');
    out
}

fn json_lines(
    values: &IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>,
    width: usize,
) -> Vec<Line<'static>> {
    let punctuation = theme().subtext();
    let mut lines = vec![Line::raw("{").style(punctuation)];

    for line in json_body(values) {
        let style = if line.null {
            theme().subtext().add_modifier(Modifier::ITALIC)
        } else {
            theme().text()
        };
        let hanging = format!("{}{INDENT}", line.indent);
        let head = match &line.key {
            Some(key) => format!("{}{key}: ", line.indent),
            None => line.indent.clone(),
        };
        let placeholder = " ".repeat(head.width());
        let options = textwrap::Options::new(width.max(hanging.width() + 1))
            .initial_indent(&placeholder)
            .subsequent_indent(&hanging);

        for (n, fragment) in textwrap::wrap(&line.body, options).into_iter().enumerate() {
            if n == 0 {
                let body = fragment[head.len().min(fragment.len())..].to_owned();
                let mut spans = vec![Span::styled(line.indent.clone(), punctuation)];
                if let Some(key) = &line.key {
                    spans.push(Span::styled(key.clone(), theme().header(line.field)));
                    spans.push(Span::styled(": ", punctuation));
                }
                spans.push(Span::styled(body, style));
                lines.push(Line::from(spans));
            } else {
                lines.push(Line::from(Span::styled(fragment.into_owned(), style)));
            }
        }
    }

    lines.push(Line::raw("}").style(punctuation));
    lines
}

fn to_json(value: &AnyValue<'_>) -> serde_json::Value {
    use serde_json::Value;
    match value {
        AnyValue::Null => Value::Null,
        AnyValue::Boolean(v) => Value::Bool(*v),
        AnyValue::Int8(v) => Value::from(*v),
        AnyValue::Int16(v) => Value::from(*v),
        AnyValue::Int32(v) => Value::from(*v),
        AnyValue::Int64(v) => Value::from(*v),
        AnyValue::UInt8(v) => Value::from(*v),
        AnyValue::UInt16(v) => Value::from(*v),
        AnyValue::UInt32(v) => Value::from(*v),
        AnyValue::UInt64(v) => Value::from(*v),
        AnyValue::Float32(v) => serde_json::Number::from_f64(*v as f64)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        AnyValue::Float64(v) => serde_json::Number::from_f64(*v)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        AnyValue::String(v) => Value::String((*v).to_owned()),
        AnyValue::StringOwned(v) => Value::String(v.to_string()),
        AnyValue::List(series) => Value::Array(series.iter().map(|item| to_json(&item)).collect()),
        other => Value::String(other.to_multi_line().into_owned()),
    }
}
