use std::{
    io::Write,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread::JoinHandle,
};

use tempfile::NamedTempFile;
use ureq::rustls::lock::Mutex;

use crate::AppResult;

#[derive(Debug)]
pub struct BackgroundDownloader {
    info: DownloadInfo,
    hndl: JoinHandle<AppResult<()>>,
}

impl BackgroundDownloader {
    pub fn new(url: String) -> Self {
        let info = DownloadInfo::default();
        BackgroundDownloader {
            info: info.clone(),
            hndl: std::thread::spawn(move || {
                let response = ureq::get(&url).call()?;
                let len = response
                    .header("Content-Length")
                    .and_then(|v| v.parse::<u64>().ok());
                if let Some(len) = len {
                    info.set_total(len);
                }
                let mut temp = NamedTempFile::new()?;
                let mut reader = response.into_reader();
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
                info.set_path(temp.path().to_owned());
                Ok(())
            }),
        }
    }

    pub fn running(&self) -> bool {
        !self.hndl.is_finished()
    }

    pub fn info(&self) -> &DownloadInfo {
        &self.info
    }
}

#[derive(Debug, Clone)]
pub struct DownloadInfo {
    pg: Arc<AtomicU64>,
    tt: Arc<AtomicU64>,
    path: Arc<Mutex<Option<PathBuf>>>,
}

impl Default for DownloadInfo {
    fn default() -> Self {
        Self {
            path: Arc::new(Mutex::new(None)),
            pg: Default::default(),
            tt: Default::default(),
        }
    }
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

    pub fn path(&self) -> Option<PathBuf> {
        self.path.lock().unwrap().clone()
    }

    fn set_path(&self, path: PathBuf) {
        *self.path.lock().unwrap() = Some(path);
    }

    pub fn is_complete(&self) -> bool {
        self.progress() == self.total()
    }
}

pub fn download_to_temp(url: &str) -> AppResult<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let response = ureq::get(url).call()?;
    std::io::copy(&mut response.into_reader(), temp.as_file_mut())?;
    Ok(temp)
}
