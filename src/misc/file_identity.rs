use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use anyhow::bail;

use crate::AppResult;

/// The identity of a regular file at the moment its path was resolved.
///
/// A path is not a stable reference to a file. Symlinks, including symlinked parent directories,
/// can be repointed between two reads, so re-reading a stored path can hand back a completely
/// different file than the one the user opened. Capturing the fully resolved target lets a later
/// read verify that the path still leads where it did, and the device/inode pair detects the file
/// being swapped while it is read.
///
/// The target comparison is the part worth relying on. The device/inode pair is best effort: a
/// file that is deleted and recreated can be handed the very same inode, and the pair is not
/// available at all outside unix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIdentity {
    target: PathBuf,
    #[cfg(unix)]
    dev: u64,
    #[cfg(unix)]
    ino: u64,
}

impl FileIdentity {
    /// Resolves `path` and captures the identity of the regular file behind it.
    ///
    /// Fails when the path cannot be resolved or does not lead to a regular file. Rejecting
    /// everything but regular files keeps a re-read from blocking forever on a fifo or consuming
    /// an endless character device such as `/dev/zero`.
    pub fn capture(path: impl AsRef<Path>) -> AppResult<Self> {
        let target = std::fs::canonicalize(path.as_ref())?;
        // Canonicalization has already expanded every symlink, so this describes the file itself.
        let meta = std::fs::symlink_metadata(&target)?;
        if !meta.is_file() {
            bail!("'{}' is not a regular file", target.to_string_lossy());
        }
        Ok(Self::from_parts(target, &meta))
    }

    #[cfg(unix)]
    fn from_parts(target: PathBuf, meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        Self {
            target,
            dev: meta.dev(),
            ino: meta.ino(),
        }
    }

    #[cfg(not(unix))]
    fn from_parts(target: PathBuf, _meta: &Metadata) -> Self {
        Self { target }
    }

    /// The fully resolved path, with every symlink expanded.
    pub fn target(&self) -> &Path {
        &self.target
    }

    /// Whether both identities resolve to the same location. A file rewritten in place, or
    /// replaced atomically under the same path, keeps its target; a repointed symlink does not.
    pub fn same_target(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_resolves_to_the_same_target_across_calls() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let first = FileIdentity::capture(file.path()).unwrap();
        let second = FileIdentity::capture(file.path()).unwrap();
        assert!(first.same_target(&second));
        assert_eq!(first, second);
    }

    #[test]
    fn capture_rejects_directories() {
        let dir = tempfile::tempdir().unwrap();
        assert!(FileIdentity::capture(dir.path()).is_err());
    }

    #[test]
    fn capture_rejects_missing_paths() {
        let dir = tempfile::tempdir().unwrap();
        assert!(FileIdentity::capture(dir.path().join("absent.csv")).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn capture_rejects_files_that_are_not_regular() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("socket");
        // A unix socket stands in for the fifos and character devices a refresh would otherwise
        // block on; they are all rejected by the same file type check.
        let _listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        assert!(FileIdentity::capture(&path).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn a_repointed_symlink_changes_the_target() {
        let dir = tempfile::tempdir().unwrap();
        let first = dir.path().join("first");
        let second = dir.path().join("second");
        let link = dir.path().join("link");
        std::fs::write(&first, "a").unwrap();
        std::fs::write(&second, "b").unwrap();
        std::os::unix::fs::symlink(&first, &link).unwrap();

        let before = FileIdentity::capture(&link).unwrap();
        std::fs::remove_file(&link).unwrap();
        std::os::unix::fs::symlink(&second, &link).unwrap();
        let after = FileIdentity::capture(&link).unwrap();

        assert!(!before.same_target(&after));
    }

    #[cfg(unix)]
    #[test]
    fn rewriting_a_file_in_place_keeps_its_identity() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let before = FileIdentity::capture(file.path()).unwrap();
        std::fs::write(file.path(), "new content").unwrap();
        let after = FileIdentity::capture(file.path()).unwrap();
        assert_eq!(before, after);
    }

    #[cfg(unix)]
    #[test]
    fn atomically_replacing_a_file_changes_its_identity_but_not_its_target() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data");
        let replacement = dir.path().join("replacement");
        std::fs::write(&path, "a").unwrap();
        std::fs::write(&replacement, "b").unwrap();
        let before = FileIdentity::capture(&path).unwrap();

        std::fs::rename(&replacement, &path).unwrap();
        let after = FileIdentity::capture(&path).unwrap();

        assert!(before.same_target(&after));
        assert_ne!(before, after);
    }
}
