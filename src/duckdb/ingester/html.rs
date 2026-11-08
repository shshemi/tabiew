use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, write_text_table},
        session::SessionConn,
    },
};
use scraper::{ElementRef, Html, Selector};
use std::path::PathBuf;

pub struct HtmlIngester {
    path: PathBuf,
}

impl HtmlIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for HtmlIngester {
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let contents = std::fs::read_to_string(&self.path)?;
        let document = Html::parse_document(&contents);
        let table = Selector::parse("table").unwrap();
        for (idx, element) in document.select(&table).enumerate() {
            let (headers, rows) = read_table(element);
            write_text_table(
                &conn,
                alias,
                &format!("html_table_{}", idx + 1),
                &headers,
                &rows,
            )?;
        }
        Ok(())
    }
}

fn read_table(table: ElementRef) -> (Vec<String>, Vec<Vec<String>>) {
    let head = Selector::parse("thead th").unwrap();
    let row = Selector::parse("tr").unwrap();
    let cell = Selector::parse("th, td").unwrap();
    let headers = table.select(&head).map(cell_text).collect::<Vec<_>>();
    let has_head = !headers.is_empty();
    let rows = table
        .select(&row)
        .filter(|tr| {
            !has_head
                || !tr
                    .ancestors()
                    .any(|node| ElementRef::wrap(node).is_some_and(|e| e.value().name() == "thead"))
        })
        .map(|tr| tr.select(&cell).map(cell_text).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if has_head {
        return (headers, rows);
    }
    let width = rows.iter().map(Vec::len).max().unwrap_or(0);
    ((1..=width).map(|i| format!("column_{i}")).collect(), rows)
}

fn cell_text(element: ElementRef) -> String {
    element.text().collect::<String>().trim().to_owned()
}
