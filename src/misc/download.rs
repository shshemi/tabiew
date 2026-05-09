use std::io::Read;

use tempfile::NamedTempFile;
use url::Url;

use crate::{AppResult, misc::http};

const CHUNK_SIZE: usize = 16_384;

pub struct Download {
    reader: Box<dyn Read + Send>,
    buffer: Box<[u8]>,
    downloaded: u64,
    total: Option<u64>,
    done: bool,
}

impl Download {
    pub fn new(url: &Url) -> AppResult<Self> {
        let total = download_size(url).ok().filter(|&n| n > 0);
        let reader = http::get(url).call()?.into_body().into_reader();
        Ok(Self {
            reader: Box::new(reader),
            buffer: vec![0u8; CHUNK_SIZE].into_boxed_slice(),
            downloaded: 0,
            total,
            done: false,
        })
    }

    pub fn next_chunk(&mut self) -> Option<&[u8]> {
        if self.done {
            return None;
        }
        match self.reader.read(&mut self.buffer) {
            Ok(0) => {
                self.done = true;
                None
            }
            Ok(n) => {
                self.downloaded += n as u64;
                Some(&self.buffer[..n])
            }
            Err(_) => {
                self.done = true;
                None
            }
        }
    }

    pub fn downloaded(&self) -> u64 {
        self.downloaded
    }

    pub fn total(&self) -> Option<u64> {
        self.total
    }

    pub fn percent(&self) -> Option<u16> {
        self.total
            .and_then(|tt| (self.downloaded * 100).checked_div(tt).map(|v| v as u16))
    }
}

impl std::fmt::Debug for Download {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Downloader")
            .field("downloaded", &self.downloaded)
            .field("total", &self.total)
            .field("done", &self.done)
            .finish()
    }
}

pub fn download_to_temp(url: &Url) -> AppResult<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let response = ureq::get(url.as_str()).call()?;
    std::io::copy(&mut response.into_body().into_reader(), temp.as_file_mut())?;
    Ok(temp)
}

pub fn download_size(url: &Url) -> AppResult<u64> {
    let respnse = http::head(url)
        .header("Accept-Encoding", "identity")
        .call()?;
    if let Some(cl) = respnse.headers().get("Content-Length") {
        let s = cl.to_str()?;
        let v = s.parse::<u64>()?;
        Ok(v)
    } else {
        Ok(0)
    }
}
