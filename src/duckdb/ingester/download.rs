use crate::{
    AppResult,
    duckdb::{
        cancel,
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
    },
    net::http,
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use url::Url;

pub(super) struct Download {
    url: Url,
    path: PathBuf,
    complete: Arc<AtomicBool>,
    inner: Box<dyn FileIngester>,
    progress: Arc<DownloadProgress>,
}

impl Download {
    pub(super) fn new(
        url: Url,
        path: PathBuf,
        complete: Arc<AtomicBool>,
        inner: Box<dyn FileIngester>,
    ) -> Self {
        let progress = Arc::new(DownloadProgress {
            download: CountProgress::new(),
            ingest: inner.progress(),
        });
        Self {
            url,
            path,
            complete,
            inner,
            progress,
        }
    }
}

impl FileIngester for Download {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let downloaded = self.fetch();
        self.complete.store(true, Ordering::Release);
        downloaded?;
        self.inner.ingest(conn, alias)
    }
}

impl Download {
    fn fetch(&self) -> AppResult<()> {
        let response = http::get(&self.url).call()?;
        let download = &self.progress.download;
        if let Some(length) = response
            .headers()
            .get("Content-Length")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok())
        {
            download.total(length);
        }
        let mut reader = response.into_body().into_reader();
        let mut file = File::create(&self.path)?;
        let mut buffer = vec![0; 65_536];
        let mut downloaded = 0;
        loop {
            cancel::check()?;
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            file.write_all(&buffer[..read])?;
            downloaded += read as u64;
            download.set(downloaded);
        }
        download.total(downloaded);
        Ok(())
    }
}

struct DownloadProgress {
    download: Arc<CountProgress>,
    ingest: Arc<dyn IngestProgress>,
}

impl IngestProgress for DownloadProgress {
    fn fraction(&self) -> Option<f64> {
        let download = self.download.fraction().unwrap_or(0.0);
        let ingest = self.ingest.fraction().unwrap_or(0.0);
        Some((download + ingest) / 2.0)
    }
}
