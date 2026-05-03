use std::{
    fmt::Debug,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread::JoinHandle,
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
                let response = ureq::get(url.as_str()).call()?;
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

    pub fn is_complete(&self) -> bool {
        self.progress() == self.total()
    }

    pub fn ratio(&self) -> Option<f64> {
        let tt = self.total();
        (tt == 0).then_some({
            let pg = self.progress() as f64;
            let tt = tt as f64;
            pg / tt
        })
    }
}

pub fn download_to_temp(url: &Url) -> AppResult<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let response = ureq::get(url.as_str()).call()?;
    std::io::copy(&mut response.into_reader(), temp.as_file_mut())?;
    Ok(temp)
}
