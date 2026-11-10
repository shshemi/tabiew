use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, TextTable},
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
        let head = Selector::parse("thead th").unwrap();
        let row = Selector::parse("tr").unwrap();
        let cell = Selector::parse("th, td").unwrap();
        for (idx, element) in document.select(&table).enumerate() {
            let mut table = TextTable::new(&conn, alias, format!("html_table_{}", idx + 1));
            for header in element.select(&head) {
                table.column(&cell_text(header));
            }
            let has_head = table.width() > 0;
            for tr in element.select(&row) {
                if has_head
                    && tr.ancestors().any(|node| {
                        ElementRef::wrap(node).is_some_and(|e| e.value().name() == "thead")
                    })
                {
                    continue;
                }
                let cells = tr.select(&cell).map(cell_text).collect::<Vec<_>>();
                while table.width() < cells.len() {
                    table.column(&format!("column_{}", table.width() + 1));
                }
                table.push(cells.into_iter().enumerate().collect())?;
            }
            table.finish()?;
        }
        Ok(())
    }
}

fn cell_text(element: ElementRef) -> String {
    element.text().collect::<String>().trim().to_owned()
}
