//! Tools for handling file system errors.
//! 
//! This module provides functions that make certain file system errors
//! less verbose to handle as well as a specialised error type.

use std::io;
use std::path::{Path, PathBuf};


/// Represents errors that occur when trying to remove files or directories.
/// 
/// Contains the underlying error returned by the operating system as well as
/// the path to the file or directory that couldn't be removed because of
/// the error.
#[derive(Debug)]
pub struct FailedToRemove {
    path: PathBuf,
    source: io::Error
}

impl FailedToRemove {
    /// Creates a new error from a path and an I/O error.
    pub fn new(path: &Path, source: io::Error) -> Self {
        FailedToRemove {
            path: path.to_path_buf(),
            source
        }
    }

    /// The path to the file or directory that couldn't be removed.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The lower-level source of this error.
    /// 
    /// It's often more convenient for a user to receive the underlying error
    /// with its original [`io::Error`] type rather than the more generic
    /// [`std::error::Error`] that [`source()`] returns.
    /// 
    /// [`source()`]: std::error::Error::source
    pub fn io_source(&self) -> &io::Error {
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

impl std::error::Error for FailedToRemove {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}


/// Does an error signal that a path wasn't found?
pub fn not_found(e: &io::Error) -> bool {
    e.kind() == io::ErrorKind::NotFound
}

/// Does an error signal that a path was unexpectedly not a directory?
pub fn not_a_directory(e: &io::Error) -> bool {
    // check against the Windows error code
    // because io::ErrorKind::NotADirectory is unstable
    matches!(e.raw_os_error(), Some(267))
}
