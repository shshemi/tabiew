use std::{
    fmt::Debug,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};

use tempfile::NamedTempFile;
use url::Url;

use crate::{
    AppResult,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
};

pub trait Reader: DataFrameReader + Debug + Send + Sync + 'static {}

impl<T> Reader for T where T: DataFrameReader + Debug + Send + Sync + 'static {}

#[derive(Debug)]
pub struct BackgroundDownloaderAndRead {
    info: DownloadInfo,
    hndl: JoinHandle<AppResult<NamedFrames>>,
}

impl BackgroundDownloaderAndRead {
    pub fn new(url: Url, df_reader: Arc<dyn Reader>) -> Self {
        let info = DownloadInfo::default();
        BackgroundDownloaderAndRead {
            info: info.clone(),
            hndl: std::thread::spawn(move || {
                info.set_total(file_size(&url)?);
                let mut reader = ureq::get(url.as_str()).call()?.into_reader();
                let mut temp = NamedTempFile::new()?;
                let writer = temp.as_file_mut();
                let mut buffer = [0_u8; 16_384];
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    info.add_progress(n as u64);
                    writer.write_all(&buffer[..n])?;
                }
                let nf =
                    df_reader.read_to_data_frames(ReaderSource::File(temp.path().to_owned()))?;
                Ok(nf)
            }),
        }
    }

    pub fn running(&self) -> bool {
        !self.hndl.is_finished()
    }

    pub fn info(&self) -> &DownloadInfo {
        &self.info
    }

    pub fn join(self) -> AppResult<NamedFrames> {
        // TODO: fix unwrap
        self.hndl.join().unwrap()
    }
}

#[derive(Debug, Default, Clone)]
pub struct DownloadInfo {
    pg: Arc<AtomicU64>,
    tt: Arc<AtomicU64>,
}

impl DownloadInfo {
    pub fn progress(&self) -> u64 {
        self.pg.load(Ordering::Relaxed)
    }

    fn add_progress(&self, pg: u64) {
        self.pg.fetch_add(pg, Ordering::Relaxed);
    }

    pub fn total(&self) -> u64 {
        self.tt.load(Ordering::Relaxed)
    }

    fn set_total(&self, tt: u64) {
        self.tt.store(tt, Ordering::Relaxed);
    }

    pub fn percent(&self) -> Option<u16> {
        let tt = self.total();
        if tt != 0 {
            let pg = self.progress() * 100;
            Some((pg / tt).min(100) as u16)
        } else {
            None
        }
    }
}

pub fn download_to_temp(url: &Url) -> AppResult<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let response = ureq::get(url.as_str()).call()?;
    std::io::copy(&mut response.into_reader(), temp.as_file_mut())?;
    Ok(temp)
}

pub fn file_size(url: &Url) -> AppResult<u64> {
    if let Ok(response) = ureq::get(url.as_str()).set("Range", "bytes=0-0").call() {
        let size = response
            .header("Content-Range")
            .and_then(|v| v.rsplit('/').next().and_then(|n| n.parse::<u64>().ok()))
            .or_else(|| {
                response
                    .header("Content-Length")
                    .and_then(|v| v.parse::<u64>().ok())
            });
        if let Some(size) = size {
            return Ok(size);
        }
    }

    if let Ok(response) = ureq::head(url.as_str())
        .set("Accept-Encoding", "identity")
        .call()
        && let Some(size) = response
            .header("Content-Length")
            .and_then(|v| v.parse::<u64>().ok())
    {
        return Ok(size);
    }

    let response = ureq::head(url.as_str()).call()?;
    Ok(response
        .header("Content-Length")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0))
}
