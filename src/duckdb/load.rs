use crate::{
    AppResult,
    duckdb::{
        DuckDB, Finished, Ingest, IngestOptions, Ingesting, Load,
        cache::file_db::FileDB,
        ingester::{Ingester, progress::Deferred},
    },
    misc::workers,
    net::http,
};
use anyhow::anyhow;
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::oneshot::{self, Receiver, error::TryRecvError};
use url::Url;

pub(super) struct Probing {
    url: Url,
    name: Option<String>,
    file: PathBuf,
    options: IngestOptions,
    max_rows: Option<usize>,
    complete: Arc<AtomicBool>,
    progress: Arc<Deferred>,
    result: Receiver<Probe>,
}

#[derive(Default, Clone)]
pub(super) struct Probe {
    validators: Option<String>,
    size: u64,
}

pub(super) fn probe(url: &Url) -> Probe {
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
    let length = header("Content-Length");
    Probe {
        validators: header("ETag").or_else(|| {
            header("Last-Modified")
                .map(|modified| format!("{modified} {}", length.clone().unwrap_or_default()))
        }),
        size: length.and_then(|length| length.parse().ok()).unwrap_or(0),
    }
}

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
        if self
            .in_memory_below
            .is_some_and(|limit| std::fs::metadata(path).is_ok_and(|meta| meta.len() < limit))
        {
            return self.ingest(
                Input::File(path.clone()),
                None,
                name,
                options,
                max_rows,
                None,
            );
        }
        if let Some(file_db) = self.cache_manager.check_existing_cache_db(path, &key)? {
            return self.ready(file_db, name.as_deref(), max_rows);
        }
        let file_db = self.cache_manager.create_new_db(path, &key)?;
        self.ingest(
            Input::File(path.clone()),
            Some(file_db),
            name,
            options,
            max_rows,
            None,
        )
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
        self.ingest(
            Input::Spooled(path),
            Some(file_db),
            Some(name),
            options,
            max_rows,
            None,
        )
    }
    pub fn load_url(
        &mut self,
        url: &Url,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
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
        let file = self.spool.path(&file_name);
        if self.part_of.is_some() {
            let probe = self
                .probed
                .get(url.as_str())
                .cloned()
                .unwrap_or_else(|| probe(url));
            return self.resolve_url(url, name, probe, file, options, max_rows, None);
        }
        let complete = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(Deferred::default());
        let (sender, result) = oneshot::channel();
        let target = url.clone();
        workers::spawn_short(move || {
            let _ = sender.send(probe(&target));
        });
        self.probing.push(Probing {
            url: url.clone(),
            name,
            file: file.clone(),
            options: options.clone(),
            max_rows,
            complete: complete.clone(),
            progress: progress.clone(),
            result,
        });
        Ok(Load::Ingesting(Ingest {
            source: PathBuf::from(url.as_str()),
            progress,
            file: Some(file),
            complete,
        }))
    }
    pub(super) fn poll_probes(&mut self, finished: &mut Finished) {
        let mut i = 0;
        while i < self.probing.len() {
            let probe = match self.probing[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(probe) => probe,
                Err(TryRecvError::Closed) => Probe::default(),
            };
            let probing = self.probing.swap_remove(i);
            let source = PathBuf::from(probing.url.as_str());
            let loaded = self.resolve_url(
                &probing.url,
                probing.name,
                probe,
                probing.file,
                &probing.options,
                probing.max_rows,
                Some(probing.complete.clone()),
            );
            match loaded {
                Ok(Load::Ingesting(ingest)) => probing.progress.set(ingest.progress),
                Ok(Load::Ready(names)) => {
                    probing.complete.store(true, Ordering::Release);
                    finished.tables.push((source, Ok(names)));
                }
                Err(e) => {
                    probing.complete.store(true, Ordering::Release);
                    finished.tables.push((source, Err(e)));
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn resolve_url(
        &mut self,
        url: &Url,
        name: Option<String>,
        probe: Probe,
        file: PathBuf,
        options: &IngestOptions,
        max_rows: Option<usize>,
        complete: Option<Arc<AtomicBool>>,
    ) -> AppResult<Load> {
        let key = serde_json::to_value(options)?;
        if let Some(validators) = &probe.validators
            && let Some(file_db) =
                self.cache_manager
                    .check_url_db(url.as_str(), validators, &key)?
        {
            return self.ready(file_db, name.as_deref(), max_rows);
        }
        let file_db = match &probe.validators {
            Some(validators) => {
                self.cache_manager
                    .create_url_db(url.as_str(), validators, probe.size, &key)?
            }
            None => FileDB::create(&file.with_extension("duckdb"), &file, &key)?,
        };
        self.ingest(
            Input::Download(url.clone(), file),
            Some(file_db),
            name,
            options,
            max_rows,
            complete,
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
        file_db: Option<FileDB>,
        name: Option<String>,
        options: &IngestOptions,
        max_rows: Option<usize>,
        complete: Option<Arc<AtomicBool>>,
    ) -> AppResult<Load> {
        let alias = self.session.attach(file_db)?;
        let (source, spooled, download) = match input {
            Input::File(path) => (path, None, None),
            Input::Spooled(path) => (PathBuf::from("-"), Some(path), None),
            Input::Download(url, path) => (PathBuf::from(url.as_str()), Some(path), Some(url)),
        };
        let file = spooled.clone().unwrap_or_else(|| source.clone());
        let complete = complete.unwrap_or_else(|| Arc::new(AtomicBool::new(download.is_none())));
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
                    source: source.clone(),
                    spooled,
                    group,
                    alias,
                    name,
                    limit: max_rows,
                    result,
                });
                Ok(Load::Ingesting(Ingest {
                    source,
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
