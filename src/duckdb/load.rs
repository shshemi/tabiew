use crate::{
    AppResult,
    duckdb::{DuckDB, Ingest, IngestOptions, Ingesting, Load, cache::file_db::FileDB, ingester::Ingester},
    net::http,
};
use anyhow::anyhow;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};
use url::Url;

enum Input {
    File(PathBuf),
    Spooled(PathBuf),
    Download(Url, PathBuf),
}

impl DuckDB {
    pub fn load(
        &mut self,
        path: &Path,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let path = &path
            .canonicalize()
            .map_err(|e| anyhow!("Failed to open {}: {}", path.display(), e))?;
        let key = serde_json::to_value(options)?;
        let name = table_name.map(String::from).or_else(|| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        });
        if let Some(file_db) = self.cache_manager.check_existing_cache_db(path, &key)? {
            return self.ready(file_db, name.as_deref(), max_rows);
        }
        let file_db = self.cache_manager.create_new_db(path, &key)?;
        self.ingest(Input::File(path.clone()), file_db, name, options, max_rows)
    }
    pub fn load_stdin(
        &mut self,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let path = self.spool.write("stdin", std::io::stdin().lock())?;
        let file_db = FileDB::create(
            &path.with_extension("duckdb"),
            &path,
            &serde_json::to_value(options)?,
        )?;
        let name = table_name.unwrap_or("Stdin").to_string();
        self.ingest(Input::Spooled(path), file_db, Some(name), options, max_rows)
    }
    pub fn load_url(
        &mut self,
        url: &Url,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let key = serde_json::to_value(options)?;
        let file_name = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .filter(|segment| !segment.is_empty())
            .unwrap_or("download")
            .to_string();
        let name = table_name.map(String::from).or_else(|| {
            Path::new(&file_name)
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        });
        let head = http::head(url)
            .header("Accept-Encoding", "identity")
            .call()
            .ok();
        let header = |name: &str| {
            head.as_ref()
                .and_then(|response| response.headers().get(name))
                .and_then(|value| value.to_str().ok())
                .map(String::from)
        };
        let validators = header("ETag").or_else(|| {
            header("Last-Modified").map(|modified| {
                format!(
                    "{modified} {}",
                    header("Content-Length").unwrap_or_default()
                )
            })
        });
        if let Some(validators) = &validators
            && let Some(file_db) =
                self.cache_manager
                    .check_url_db(url.as_str(), validators, &key)?
        {
            return self.ready(file_db, name.as_deref(), max_rows);
        }
        let file = self.spool.path(&file_name);
        let file_db = match &validators {
            Some(validators) => {
                let size = header("Content-Length")
                    .and_then(|length| length.parse().ok())
                    .unwrap_or(0);
                self.cache_manager
                    .create_url_db(url.as_str(), validators, size, &key)?
            }
            None => FileDB::create(&file.with_extension("duckdb"), &file, &key)?,
        };
        self.ingest(
            Input::Download(url.clone(), file),
            file_db,
            name,
            options,
            max_rows,
        )
    }
    pub(super) fn ready(
        &mut self,
        file_db: FileDB,
        name: Option<&str>,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let alias = self.session.attach(Some(file_db))?;
        if self.join(&alias, name, false) {
            return Ok(Load::Ready(Vec::new()));
        }
        let names = self.views(&alias, name, max_rows);
        self.session.unpin(&alias);
        names.map(Load::Ready)
    }
    fn ingest(
        &mut self,
        input: Input,
        file_db: FileDB,
        name: Option<String>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let alias = self.session.attach(Some(file_db))?;
        let (source, spooled, download) = match input {
            Input::File(path) => (path, None, None),
            Input::Spooled(path) => (PathBuf::from("-"), Some(path), None),
            Input::Download(url, path) => (PathBuf::from(url.as_str()), Some(path), Some(url)),
        };
        let file = spooled.clone().unwrap_or_else(|| source.clone());
        let complete = Arc::new(AtomicBool::new(download.is_none()));
        let ingested = Ingester::new(
            self.session.session()?,
            alias.clone(),
            file.clone(),
            options,
        )
        .map(|ingester| match download {
            Some(url) => ingester.download(url, file.clone(), complete.clone()),
            None => ingester,
        })
        .and_then(Ingester::ingest);
        match ingested {
            Ok((progress, result)) => {
                let group = self.part_of;
                self.join(&alias, name.as_deref(), true);
                self.ingesting.push(Ingesting {
                    source,
                    spooled,
                    group,
                    alias,
                    name,
                    limit: max_rows,
                    result,
                });
                Ok(Load::Ingesting(Ingest {
                    progress,
                    file: Some(file),
                    complete,
                }))
            }
            Err(e) => {
                self.session.unpin(&alias);
                Err(e)
            }
        }
    }
}
