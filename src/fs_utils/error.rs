use std::io;
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct FailedToRemove {
    path: PathBuf,
    source: io::Error
}

impl FailedToRemove {
    pub fn new(path: &Path, source: io::Error) -> Self {
        FailedToRemove {
            path: path.to_path_buf(),
            source
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn source(&self) -> &io::Error {
        &self.source
    }
}

impl std::fmt::Display for FailedToRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to remove \"{}\": {}",
            self.path.display(),
            self.source
        )
    }
}

impl std::error::Error for FailedToRemove {}


/// Checks whether an IO error is of the kind NotFound.
///
/// # Examples
///
/// ```
/// use std::io;
/// use folder_cleaner::fs_utils::error::not_found;
/// 
/// let e = io::Error::from(io::ErrorKind::NotFound);
/// assert_eq!(not_found(&e), true);
/// ```
pub fn not_found(e: &io::Error) -> bool {
    e.kind() == io::ErrorKind::NotFound
}

/// Checks whether an IO error is of the kind NotADirectory.
/// 
/// [`std::io::ErrorKind::NotADirectory`] is currently unstable and thus can't be
/// used directly in stable Rust, but it is still sometimes emitted by
/// the standard library.
///
/// # Examples
///
/// ```
/// use std::io;
/// use folder_cleaner::fs_utils::error::not_a_directory;
///
/// let e = io::Error::from_raw_os_error(267);
/// assert_eq!(not_a_directory(&e), true);
/// ```
pub fn not_a_directory(e: &io::Error) -> bool {
    // check against the Windows error code
    // because io::ErrorKind::NotADirectory is unstable
    matches!(e.raw_os_error(), Some(267))
}
