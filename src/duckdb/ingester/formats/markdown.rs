use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, TextTable, progress::CountProgress},
        session::SessionConn,
    },
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::{path::PathBuf, sync::Arc};

pub struct MarkdownIngester {
    path: PathBuf,
    types: Vec<String>,
    progress: Arc<CountProgress>,
}

impl MarkdownIngester {
    pub fn new(path: PathBuf, types: Vec<String>) -> Self {
        Self {
            path,
            types,
            progress: CountProgress::new(),
        }
    }
}

impl FileIngester for MarkdownIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let contents = std::fs::read_to_string(&self.path)?;
        self.progress.total(contents.len() as u64);
        let mut tables = 0;
        let mut table = None;
        let mut row = Vec::new();
        let mut cell = String::new();
        let mut in_head = false;
        let mut in_cell = false;
        for (event, range) in Parser::new_ext(&contents, Options::ENABLE_TABLES).into_offset_iter()
        {
            self.progress.set(range.end as u64);
            match event {
                Event::Start(Tag::Table(_)) => {
                    tables += 1;
                    table = Some(TextTable::new(
                        &conn,
                        alias,
                        format!("markdown_table_{tables}"),
                        &self.types,
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
