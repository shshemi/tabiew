use std::{path::Path, sync::Arc};

use anyhow::bail;
use itertools::Itertools;
use polars::frame::DataFrame;

use crate::{
    AppResult,
    io::reader::{NamedFrames, ReaderSource},
    misc::{file_identity::FileIdentity, remote_load::Reader},
};

/// Everything needed to reproduce a single registered table from the file it was loaded from.
///
/// The frame is identified by the name and position it had in the original read, not by the name
/// it was registered under: registration appends a `_N` suffix to keep table names unique, and
/// matching that suffix back against the source can land on an unrelated frame.
#[derive(Debug, Clone)]
pub struct RefreshSource {
    reader: Arc<dyn Reader>,
    identity: FileIdentity,
    frame_name: String,
    frame_index: usize,
    frame_count: usize,
}

impl RefreshSource {
    pub fn new(
        reader: Arc<dyn Reader>,
        identity: FileIdentity,
        frame_name: impl Into<String>,
        frame_index: usize,
        frame_count: usize,
    ) -> Self {
        Self {
            reader,
            identity,
            frame_name: frame_name.into(),
            frame_index,
            frame_count,
        }
    }

    pub fn identity(&self) -> &FileIdentity {
        &self.identity
    }
}

/// Re-reads `path` and returns the frame this source was built from.
///
/// The path is resolved before and after the read: the first check refuses a path that no longer
/// leads to the file the user opened (a repointed symlink, or a symlinked parent directory), and
/// the second rejects the result if the file was replaced while it was being read. Both checks
/// also reject anything that is not a regular file.
pub fn read(path: &Path, source: &RefreshSource) -> AppResult<DataFrame> {
    let before = FileIdentity::capture(path)?;
    if !before.same_target(source.identity()) {
        bail!(
            "'{}' now resolves to '{}' instead of '{}'",
            path.to_string_lossy(),
            before.target().to_string_lossy(),
            source.identity().target().to_string_lossy(),
        );
    }
    // Read through the original path so that frames keep the names they had on the first read;
    // the identity check below covers the window between resolving and reading it.
    let frames = source
        .reader
        .read_to_data_frames(ReaderSource::File(path.to_owned()))?;
    if FileIdentity::capture(path)? != before {
        bail!("the file was replaced while it was being read");
    }
    select_frame(frames, source)
}

/// Picks the frame that corresponds to `source` among the freshly read `frames`.
///
/// A name match is the only accepted answer, except for sources that held a single frame both
/// then and now, where there is nothing to confuse it with. When a source yields several frames
/// with the same name, the recorded position breaks the tie. Anything else is ambiguous and is
/// reported instead of guessed: substituting an unrelated frame would show one table's rows under
/// another table's name.
fn select_frame(frames: NamedFrames, source: &RefreshSource) -> AppResult<DataFrame> {
    let mut frames = frames.into_vec();
    let matches = frames
        .iter()
        .positions(|(name, _)| name == &source.frame_name)
        .collect_vec();
    match matches.as_slice() {
        [idx] => Ok(frames.swap_remove(*idx).1),
        [] if source.frame_count == 1 && frames.len() == 1 => Ok(frames.swap_remove(0).1),
        [] => bail!(
            "'{}' no longer contains a frame named '{}'",
            source.identity().target().to_string_lossy(),
            source.frame_name,
        ),
        _ if matches.contains(&source.frame_index) => Ok(frames.swap_remove(source.frame_index).1),
        _ => bail!(
            "'{}' contains {} frames named '{}' and none of them is at the expected position",
            source.identity().target().to_string_lossy(),
            matches.len(),
            source.frame_name,
        ),
    }
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::*;
    use crate::io::reader::CsvToDataFrame;

    fn source(frame_name: &str, frame_index: usize, frame_count: usize) -> RefreshSource {
        let file = tempfile::NamedTempFile::new().unwrap();
        RefreshSource::new(
            Arc::new(CsvToDataFrame::default()),
            FileIdentity::capture(file.path()).unwrap(),
            frame_name,
            frame_index,
            frame_count,
        )
    }

    fn frames(names: &[&str]) -> NamedFrames {
        names
            .iter()
            .enumerate()
            .map(|(idx, name)| (name.to_string(), df!("a" => [idx as i64]).unwrap()))
            .collect()
    }

    #[test]
    fn select_frame_matches_by_name() {
        let df = select_frame(frames(&["users", "orders"]), &source("orders", 1, 2)).unwrap();
        assert_eq!(df, df!("a" => [1i64]).unwrap());
    }

    #[test]
    fn select_frame_matches_a_moved_frame_by_name() {
        let df = select_frame(frames(&["orders", "users"]), &source("orders", 1, 2)).unwrap();
        assert_eq!(df, df!("a" => [0i64]).unwrap());
    }

    #[test]
    fn select_frame_matches_the_source_name_not_the_registered_name() {
        // A second file holding `sales` is registered as `sales_2`; the refresh must still follow
        // the frame actually loaded, even when the file also holds a literal `sales_2`.
        let df = select_frame(frames(&["sales", "sales_2"]), &source("sales", 0, 2)).unwrap();
        assert_eq!(df, df!("a" => [0i64]).unwrap());
    }

    #[test]
    fn select_frame_takes_the_only_frame_of_a_single_frame_source() {
        let df = select_frame(frames(&["renamed"]), &source("original", 0, 1)).unwrap();
        assert_eq!(df, df!("a" => [0i64]).unwrap());
    }

    #[test]
    fn select_frame_refuses_an_unmatched_name_in_a_multi_frame_source() {
        let err = select_frame(frames(&["users", "orders"]), &source("sales", 0, 2)).unwrap_err();
        assert!(err.to_string().contains("no longer contains a frame"));
    }

    #[test]
    fn select_frame_refuses_a_shrunk_multi_frame_source() {
        // The source used to hold several frames and now holds one under a different name: taking
        // it would show another table's rows under this table's name.
        let err = select_frame(frames(&["payments"]), &source("users", 0, 2)).unwrap_err();
        assert!(err.to_string().contains("no longer contains a frame"));
    }

    #[test]
    fn select_frame_refuses_an_empty_read() {
        assert!(select_frame(frames(&[]), &source("users", 0, 1)).is_err());
    }

    #[test]
    fn select_frame_breaks_duplicate_names_with_the_recorded_position() {
        let df = select_frame(frames(&["dup", "dup"]), &source("dup", 1, 2)).unwrap();
        assert_eq!(df, df!("a" => [1i64]).unwrap());
    }

    #[test]
    fn select_frame_refuses_duplicate_names_at_unexpected_positions() {
        let err = select_frame(frames(&["dup", "dup", "other"]), &source("dup", 2, 3)).unwrap_err();
        assert!(err.to_string().contains("expected position"));
    }

    #[test]
    fn read_reloads_a_file_rewritten_in_place() {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), "a,b\n1,2\n").unwrap();
        let source = RefreshSource::new(
            Arc::new(CsvToDataFrame::default()),
            FileIdentity::capture(file.path()).unwrap(),
            "whatever",
            0,
            1,
        );

        std::fs::write(file.path(), "a,b\n1,2\n3,4\n5,6\n").unwrap();
        let df = read(file.path(), &source).unwrap();

        assert_eq!(df.height(), 3);
    }

    #[cfg(unix)]
    #[test]
    fn read_refuses_a_path_that_now_resolves_elsewhere() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join("data.csv");
        let other = dir.path().join("other.csv");
        let link = dir.path().join("link.csv");
        std::fs::write(&data, "a,b\n1,2\n").unwrap();
        std::fs::write(&other, "secret\nvalue\n").unwrap();
        std::os::unix::fs::symlink(&data, &link).unwrap();

        let source = RefreshSource::new(
            Arc::new(CsvToDataFrame::default()),
            FileIdentity::capture(&link).unwrap(),
            "data",
            0,
            1,
        );

        std::fs::remove_file(&link).unwrap();
        std::os::unix::fs::symlink(&other, &link).unwrap();

        let err = read(&link, &source).unwrap_err();
        assert!(err.to_string().contains("now resolves to"));
    }

    #[cfg(unix)]
    #[test]
    fn read_refuses_a_source_replaced_by_a_non_regular_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data.csv");
        std::fs::write(&path, "a,b\n1,2\n").unwrap();
        let source = RefreshSource::new(
            Arc::new(CsvToDataFrame::default()),
            FileIdentity::capture(&path).unwrap(),
            "data",
            0,
            1,
        );

        std::fs::remove_file(&path).unwrap();
        let _listener = std::os::unix::net::UnixListener::bind(&path).unwrap();

        assert!(read(&path, &source).is_err());
    }
}
