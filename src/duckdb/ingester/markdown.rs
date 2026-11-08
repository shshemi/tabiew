use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, write_text_table},
        session::SessionConn,
    },
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::path::PathBuf;

pub struct MarkdownIngester {
    path: PathBuf,
}

impl MarkdownIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for MarkdownIngester {
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let contents = std::fs::read_to_string(&self.path)?;
        let parser = Parser::new_ext(&contents, Options::ENABLE_TABLES);
        for (idx, (headers, rows)) in read_tables(parser).into_iter().enumerate() {
            write_text_table(
                &conn,
                alias,
                &format!("markdown_table_{}", idx + 1),
                &headers,
                &rows,
            )?;
        }
        Ok(())
    }
}

fn read_tables(parser: Parser) -> Vec<(Vec<String>, Vec<Vec<String>>)> {
    let mut tables = Vec::new();
    let mut headers = Vec::new();
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cell = String::new();
    let mut in_head = false;
    let mut in_cell = false;
    for event in parser {
        match event {
            Event::Start(Tag::Table(_)) => {
                headers.clear();
                rows.clear();
            }
            Event::End(TagEnd::Table) => {
                tables.push((std::mem::take(&mut headers), std::mem::take(&mut rows)));
            }
            Event::Start(Tag::TableHead) => in_head = true,
            Event::End(TagEnd::TableHead) => in_head = false,
            Event::Start(Tag::TableRow) => row.clear(),
            Event::End(TagEnd::TableRow) if !in_head => rows.push(std::mem::take(&mut row)),
            Event::Start(Tag::TableCell) => {
                in_cell = true;
                cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                in_cell = false;
                let value = std::mem::take(&mut cell).trim().to_owned();
                if in_head {
                    headers.push(value);
                } else {
                    row.push(value);
                }
            }
            Event::Text(text) | Event::Code(text) if in_cell => cell.push_str(&text),
            Event::SoftBreak | Event::HardBreak if in_cell => cell.push(' '),
            _ => {}
        }
    }
    tables
}
