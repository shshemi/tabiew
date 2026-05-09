use std::{fs::File, io::Write};

use polars::{frame::DataFrame, prelude::AnyValue};

use crate::{AppResult, misc::osc52::CopyToClipboardOsc52};

use super::traits::{Destination, WriteToFile};

#[derive(Debug, Default)]
pub struct WriteToMarkdown;

impl WriteToFile for WriteToMarkdown {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        let buf = render_markdown(data_frame);
        match dest {
            Destination::File(path) => {
                File::create(path)?.write_all(buf.as_bytes())?;
                Ok(())
            }
            Destination::Clipboard => {
                buf.into_bytes().copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}

fn render_markdown(df: &DataFrame) -> String {
    let columns = df.columns();
    let height = df.height();

    let headers: Vec<String> = columns
        .iter()
        .map(|c| escape_cell(c.name().as_str()))
        .collect();

    let mut out = String::new();

    out.push('|');
    for h in &headers {
        out.push(' ');
        out.push_str(h);
        out.push_str(" |");
    }
    out.push('\n');

    out.push('|');
    for _ in &headers {
        out.push_str("---|");
    }
    out.push('\n');

    for i in 0..height {
        out.push('|');
        for col in columns {
            let cell = match col.get(i) {
                Ok(v) => escape_cell(&format_value(&v)),
                Err(_) => String::new(),
            };
            out.push(' ');
            out.push_str(&cell);
            out.push_str(" |");
        }
        out.push('\n');
    }

    out
}

fn format_value(v: &AnyValue<'_>) -> String {
    match v {
        AnyValue::String(s) => (*s).to_string(),
        AnyValue::StringOwned(s) => s.to_string(),
        _ => v.to_string(),
    }
}

fn escape_cell(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\r', "")
        .replace('\n', "<br>")
}
