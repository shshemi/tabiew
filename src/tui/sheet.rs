use std::{collections::BTreeMap, ops::Add};

use crossterm::event::{KeyCode, KeyModifiers};
use polars::{frame::DataFrame, prelude::AnyValue};
use serde::Serialize;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::Span,
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    handler::message::Message,
    misc::{buffer_ext::BufferExt, config::theme, osc52::CopyToClipboardOsc52},
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
        misc::{any_value_formatter::AnyValueFormatter, text_builder::TextBuilder},
        tag_line::{Tag, TagLine},
        utils::Scroll,
    },
};

#[derive(Debug)]
pub struct Sheet {
    scroll: Scroll,
    row: Option<usize>,
    // values: IndexMap<PlSmallStr, (AnyValue<'static>, DataType)>,
    df: DataFrame,
    format: Format,
    value: Value,
}

impl Sheet {
    pub fn new(df: DataFrame, row: impl Into<Option<usize>>) -> Self {
        let row = row.into();
        let value = row.and_then(|idx| get_row(&df, idx)).unwrap_or(Value::Null);
        Self {
            scroll: Default::default(),
            row,
            df,
            format: Default::default(),
            value,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll.up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll.down();
    }

    pub fn row(&self) -> Option<usize> {
        self.row
    }

    pub fn set_row(&mut self, row: impl Into<Option<usize>>) {
        self.row = row.into();
        self.sync_value();
    }

    pub fn set_data_frame(&mut self, df: DataFrame) {
        self.df = df;
        self.sync_value();
    }

    fn sync_value(&mut self) {
        self.value = self
            .row
            .and_then(|idx| get_row(&self.df, idx))
            .unwrap_or(Value::Null);
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

        if let Some(row) = self.row
            && let Some(value) = get_row(&self.df, row)
        {
            let mut tb = TextBuilder::default();
            match self.format {
                Format::Json => styled_json(&mut tb, value, 0),
                Format::Yaml => styled_yaml(&mut tb, value, 0),
            };
            let pg = Paragraph::new(tb.into_text())
                .block(
                    Block::app_default()
                        .app_title(
                            self.row()
                                .map(|row| format!("Row {}", row.add(1)))
                                .unwrap_or_default(),
                        )
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
                )
                .wrap(Wrap { trim: false });

            self.scroll
                .adjust(pg.line_count(area.width), area.height.saturating_sub(2));

            pg.scroll((self.scroll.val_u16(), 0)).render(area, buf);
        }
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
                if let Some(row) = self.row
                    && let Some(value) = get_row(&self.df, row)
                    && let Ok(text) = serde_json::to_string_pretty(&value)
                {
                    text.copy_to_clipboard_via_osc52();
                    Message::AppShowToast(format!("Row #{} copied to clipboard", row + 1))
                        .enqueue();
                }
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Format {
    #[default]
    Json,
    Yaml,
}

impl Format {
    fn toggled(self) -> Self {
        match self {
            Format::Yaml => Format::Json,
            Format::Json => Format::Yaml,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(untagged)]
enum Number {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

impl Eq for Number {}

impl Value {
    fn int(i: i64) -> Self {
        if i < 0 {
            Value::Number(Number::NegInt(i))
        } else {
            Value::Number(Number::PosInt(i as u64))
        }
    }

    fn float(f: f64) -> Self {
        if f.is_finite() {
            Value::Number(Number::Float(f))
        } else {
            Value::Null
        }
    }
}

fn get_row(df: &DataFrame, idx: usize) -> Option<Value> {
    let fields = df.fields();
    let values = df.get(idx)?;
    Some(any_value_to_value(AnyValue::StructOwned(Box::new((
        values, fields,
    )))))
}

fn any_value_to_value(any_value: AnyValue) -> Value {
    match any_value {
        AnyValue::Null => Value::Null,
        AnyValue::Boolean(b) => Value::Bool(b),
        AnyValue::String(s) => Value::String(s.to_owned()),
        AnyValue::StringOwned(s) => Value::String(s.into_string()),
        AnyValue::UInt8(u) => Value::Number(Number::PosInt(u.into())),
        AnyValue::UInt16(u) => Value::Number(Number::PosInt(u.into())),
        AnyValue::UInt32(u) => Value::Number(Number::PosInt(u.into())),
        AnyValue::UInt64(u) => Value::Number(Number::PosInt(u)),
        AnyValue::UInt128(u) => u64::try_from(u).map_or_else(
            |_| Value::String(u.to_string()),
            |u| Value::Number(Number::PosInt(u)),
        ),
        AnyValue::Int8(i) => Value::int(i.into()),
        AnyValue::Int16(i) => Value::int(i.into()),
        AnyValue::Int32(i) => Value::int(i.into()),
        AnyValue::Int64(i) => Value::int(i),
        AnyValue::Int128(i) => {
            i64::try_from(i).map_or_else(|_| Value::String(i.to_string()), Value::int)
        }
        AnyValue::Float16(f) => Value::float(f.into()),
        AnyValue::Float32(f) => Value::float(f.into()),
        AnyValue::Float64(f) => Value::float(f),
        AnyValue::List(series) | AnyValue::Array(series, _) => {
            Value::Array(series.iter().map(any_value_to_value).collect())
        }
        AnyValue::Struct(_, _, fields) => Value::Object(
            fields
                .iter()
                .map(|f| f.name().to_string())
                .zip(any_value._iter_struct_av().map(any_value_to_value))
                .collect(),
        ),
        AnyValue::StructOwned(s) => {
            let (values, fields) = *s;
            Value::Object(
                fields
                    .into_iter()
                    .map(|f| f.name().to_string())
                    .zip(values.into_iter().map(any_value_to_value))
                    .collect(),
            )
        }
        av => Value::String(
            AnyValueFormatter::new(None)
                .into_multi_line(av)
                .into_owned(),
        ),
    }
}

fn styled_json(tb: &mut TextBuilder, value: Value, lvl: usize) {
    match value {
        Value::Null => tb.push_span(Span::styled(
            "null",
            theme().subtext().add_modifier(Modifier::ITALIC),
        )),
        Value::Bool(b) => tb.push_span(styled_json_text(b.to_string())),
        Value::Number(Number::PosInt(u)) => tb.push_span(styled_json_text(u.to_string())),
        Value::Number(Number::NegInt(i)) => tb.push_span(styled_json_text(i.to_string())),
        Value::Number(Number::Float(f)) => tb.push_span(styled_json_text(
            serde_json::Number::from_f64(f).map_or_else(|| f.to_string(), |n| n.to_string()),
        )),
        Value::String(s) => tb.push_span(styled_json_text(styled_json_quoted(&s))),
        Value::Array(arr) if arr.is_empty() => tb.push_span(styled_json_punctuation("[]")),
        Value::Object(map) if map.is_empty() => tb.push_span(styled_json_punctuation("{}")),
        Value::Array(arr) => {
            tb.push_span(styled_json_punctuation("["));
            let last = arr.len() - 1;
            for (idx, v) in arr.into_iter().enumerate() {
                tb.new_line();
                tb.push_span(styled_json_indent(lvl + 1));
                styled_json(tb, v, lvl + 1);
                if idx != last {
                    tb.push_span(styled_json_punctuation(","));
                }
            }
            tb.new_line();
            tb.push_span(styled_json_indent(lvl));
            tb.push_span(styled_json_punctuation("]"));
        }
        Value::Object(map) => {
            tb.push_span(styled_json_punctuation("{"));
            let last = map.len() - 1;
            for (idx, (k, v)) in map.into_iter().enumerate() {
                tb.new_line();
                tb.push_span(styled_json_indent(lvl + 1));
                tb.push_span(Span::styled(styled_json_quoted(&k), theme().header(idx)));
                tb.push_span(styled_json_punctuation(": "));
                styled_json(tb, v, lvl + 1);
                if idx != last {
                    tb.push_span(styled_json_punctuation(","));
                }
            }
            tb.new_line();
            tb.push_span(styled_json_indent(lvl));
            tb.push_span(styled_json_punctuation("}"));
        }
    }
}

fn styled_json_punctuation(text: &'static str) -> Span<'static> {
    Span::styled(text, theme().subtext())
}

fn styled_json_indent(lvl: usize) -> Span<'static> {
    Span::styled("  ".repeat(lvl), theme().subtext())
}

fn styled_json_text(text: String) -> Span<'static> {
    Span::styled(text, theme().text())
}

fn styled_json_quoted(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{s}\""))
}

fn styled_yaml(tb: &mut TextBuilder, value: Value, lvl: usize) {
    match value {
        Value::Null => tb.push_span(Span::styled(
            "null",
            theme().subtext().add_modifier(Modifier::ITALIC),
        )),
        Value::Bool(b) => tb.push_span(styled_yaml_text(b.to_string())),
        Value::Number(Number::PosInt(u)) => tb.push_span(styled_yaml_text(u.to_string())),
        Value::Number(Number::NegInt(i)) => tb.push_span(styled_yaml_text(i.to_string())),
        Value::Number(Number::Float(f)) => tb.push_span(styled_yaml_text(
            serde_json::Number::from_f64(f).map_or_else(|| f.to_string(), |n| n.to_string()),
        )),
        Value::String(s) => tb.push_span(styled_yaml_text(styled_yaml_quoted(&s))),
        Value::Array(arr) if arr.is_empty() => tb.push_span(styled_yaml_punctuation("[]")),
        Value::Object(map) if map.is_empty() => tb.push_span(styled_yaml_punctuation("{}")),
        Value::Array(arr) => {
            for (idx, v) in arr.into_iter().enumerate() {
                if idx != 0 {
                    tb.new_line();
                    tb.push_span(styled_yaml_indent(lvl));
                }
                tb.push_span(styled_yaml_punctuation("- "));
                styled_yaml(tb, v, lvl + 1);
            }
        }
        Value::Object(map) => {
            for (idx, (k, v)) in map.into_iter().enumerate() {
                if idx != 0 {
                    tb.new_line();
                    tb.push_span(styled_yaml_indent(lvl));
                }
                tb.push_span(Span::styled(styled_yaml_quoted(&k), theme().header(idx)));
                tb.push_span(styled_yaml_punctuation(":"));
                if styled_yaml_is_block(&v) {
                    tb.new_line();
                    tb.push_span(styled_yaml_indent(lvl + 1));
                } else {
                    tb.push_span(styled_yaml_punctuation(" "));
                }
                styled_yaml(tb, v, lvl + 1);
            }
        }
    }
}

fn styled_yaml_punctuation(text: &'static str) -> Span<'static> {
    Span::styled(text, theme().subtext())
}

fn styled_yaml_indent(lvl: usize) -> Span<'static> {
    Span::styled("  ".repeat(lvl), theme().subtext())
}

fn styled_yaml_text(text: String) -> Span<'static> {
    Span::styled(text, theme().text())
}

fn styled_yaml_is_block(value: &Value) -> bool {
    match value {
        Value::Array(arr) => !arr.is_empty(),
        Value::Object(map) => !map.is_empty(),
        _ => false,
    }
}

fn styled_yaml_quoted(s: &str) -> String {
    let lower = s.to_lowercase();
    let reserved = matches!(
        lower.as_str(),
        "null" | "~" | "true" | "false" | "yes" | "no" | "on" | "off" | "y" | "n"
    ) || matches!(lower.as_str(), ".inf" | "-.inf" | "+.inf" | ".nan")
        || lower.starts_with("0x")
        || lower.starts_with("0o")
        || s.parse::<f64>().is_ok();
    let special = s.is_empty()
        || s.starts_with(|c: char| "-?:,[]{}#&*!|>'\"%@`".contains(c) || c.is_whitespace())
        || s.ends_with(|c: char| c == ':' || c.is_whitespace())
        || s.contains(": ")
        || s.contains(" #")
        || s.contains(char::is_control);
    if reserved || special {
        styled_json_quoted(s)
    } else {
        s.to_owned()
    }
}
