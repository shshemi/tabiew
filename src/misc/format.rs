use crate::args::Format;
use std::path::Path;

pub fn resolve_format(format: Option<&Format>, path: &Path) -> Format {
    if let Some(format) = format {
        return format.clone();
    }
    let extension = path
        .extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    match extension.as_str() {
        "tsv" => Format::Tsv,
        "parquet" | "pqt" => Format::Parquet,
        "json" => Format::Json,
        "jsonl" => Format::Jsonl,
        "arrow" => Format::Arrow,
        "avro" => Format::Avro,
        "fwf" => Format::Fwf,
        "db" | "sqlite" => Format::Sqlite,
        "xls" | "xlsx" | "xlsm" | "xlsb" => Format::Excel,
        "html" | "htm" => Format::Html,
        "md" | "markdown" => Format::Markdown,
        _ => Format::Csv,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_overrides_the_extension_and_csv_is_the_default() {
        let resolve =
            |path, format: Option<Format>| resolve_format(format.as_ref(), Path::new(path));
        assert_eq!(resolve("sales.CSV", None), Format::Csv);
        assert_eq!(resolve("data.tsv", None), Format::Tsv);
        assert_eq!(resolve("notes.md", None), Format::Markdown);
        assert_eq!(resolve("page.htm", None), Format::Html);
        assert_eq!(resolve("shop.db", None), Format::Sqlite);
        assert_eq!(resolve("book.XLSX", None), Format::Excel);
        assert_eq!(resolve("events.jsonl", None), Format::Jsonl);
        assert_eq!(resolve("sales.txt", None), Format::Csv);
        assert_eq!(resolve("sales", None), Format::Csv);
        assert_eq!(resolve("sales.csv", Some(Format::Parquet)), Format::Parquet);
    }
}
