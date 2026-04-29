use tempfile::NamedTempFile;

use crate::AppResult;

pub fn download_to_temp(url: &str) -> AppResult<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let response = ureq::get(url).call()?;
    std::io::copy(&mut response.into_reader(), temp.as_file_mut())?;
    Ok(temp)
}
