use std::{
    fmt::Debug,
    io::{Read, Write},
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
    misc::{download::download_size, http},
};

pub trait Reader: DataFrameReader + Debug + Send + Sync + 'static {}

impl<T> Reader for T where T: DataFrameReader + Debug + Send + Sync + 'static {}

#[derive(Debug)]
pub struct RemoteLoad {
    info: DownloadInfo,
    hndl: JoinHandle<AppResult<NamedFrames>>,
}

impl RemoteLoad {
    pub fn new(url: Url, df_reader: Arc<dyn Reader>) -> Self {
        let info = DownloadInfo::default();
        RemoteLoad {
            info: info.clone(),
            hndl: std::thread::spawn(move || {
                info.set_total(download_size(&url)?);
                if info.total() == 0 {
                    todo!() // Just testing
                }
                let mut reader = http::get(&url).call()?.into_body().into_reader();
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
        let pg = self.progress() * 100;
        let tt = self.total();
        pg.checked_div(tt).map(|u| u as u16)
    }
}
