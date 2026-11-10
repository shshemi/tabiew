use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, TextTable},
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
        let mut tables = 0;
        let mut table = None;
        let mut row = Vec::new();
        let mut cell = String::new();
        let mut in_head = false;
        let mut in_cell = false;
        for event in Parser::new_ext(&contents, Options::ENABLE_TABLES) {
            match event {
                Event::Start(Tag::Table(_)) => {
                    tables += 1;
                    table = Some(TextTable::new(
                        &conn,
                        alias,
                        format!("markdown_table_{tables}"),
                    ));
                }
                Event::End(TagEnd::Table) => {
                    if let Some(table) = table.take() {
                        table.finish()?;
                    }
                }
                Event::Start(Tag::TableHead) => in_head = true,
                Event::End(TagEnd::TableHead) => in_head = false,
                Event::Start(Tag::TableRow) => row.clear(),
                Event::End(TagEnd::TableRow) if !in_head => {
                    if let Some(table) = table.as_mut() {
                        table.push(std::mem::take(&mut row).into_iter().enumerate().collect())?;
                    }
                }
                Event::Start(Tag::TableCell) => {
                    in_cell = true;
                    cell.clear();
                }
                Event::End(TagEnd::TableCell) => {
                    in_cell = false;
                    let value = std::mem::take(&mut cell).trim().to_owned();
                    match table.as_mut() {
                        Some(table) if in_head => {
                            table.column(&value);
                        }
                        _ => row.push(value),
                    }
                }
                Event::Text(text) | Event::Code(text) if in_cell => cell.push_str(&text),
                Event::SoftBreak | Event::HardBreak if in_cell => cell.push(' '),
                _ => {}
            }
        }
        Ok(())
    }
}
