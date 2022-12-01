//! Tools for routinely cleaning folders.
//! 
//! This module contains tools for creating and executing routines that
//! periodically remove files and subdirectories from a specific directory
//! based on a pattern.
//! 
//! # Examples
//! 
//! Creates a routine to delete all contents of a user's Downloads folder
//! hourly and creates a [thread] that runs the routine for as long as the
//! program is allowed to run.
//! 
//! ```no_run
//! use folder_cleaner::fs_utils::FilePattern;
//! use folder_cleaner::routine::{Routine, spawn_routine};
//! use std::path::PathBuf;
//! use time::Duration;
//! 
//! let downloads_routine = Routine {
//!     directory: PathBuf::from(r"C:\Users\user\Downloads"),
//!     interval: Duration::HOUR,
//!     pattern: FilePattern::Any
//! };
//! 
//! let downloads_thread = spawn_routine(downloads_routine);
//! downloads_thread.join().unwrap();
//! ```

use std::path::PathBuf;
use std::thread;
use time::Duration;

use crate::fs_utils::{self, FilePattern};


/// A routine to clear a directory based on a pattern.
/// 
/// Can be [`run`](Self::run()) to clear the directory once.
/// Roughly the amount of time represented by the `interval` of a routine
/// should be allowed to pass between repeated, automated runs.
/// 
/// More details about using this type can be found in the
/// [`module documentation`](crate::routine).
/// 
/// # Examples
/// 
/// Creates a routine to remove any shortcuts from a user's desktop hourly
/// and runs it once.
/// 
/// ```no_run
/// use folder_cleaner::fs_utils::FilePattern;
/// use folder_cleaner::routine::Routine;
/// use std::path::PathBuf;
/// use time::Duration;
/// 
/// let desktop_routine = Routine {
///     directory: PathBuf::from(r"C:\Users\user\Desktop"),
///     interval: Duration::HOUR,
///     pattern: FilePattern::Extension("lnk".into())
/// };
/// 
/// desktop_routine.run();
/// ```
pub struct Routine {
    pub directory: PathBuf,
    pub interval: Duration,
    pub pattern: FilePattern
}

impl Routine {
    /// Executes a routine once.
    /// 
    /// Any files and directories in the routine's `directory` matching
    /// the routine's `pattern` are removed. See [`FilePattern`] and
    /// [`remove`](fs_utils::remove).
    /// 
    /// # Errors
    /// 
    /// This function returns an error if the routine's `directory` can't be
    /// accessed, for example if it doesn't exist or if the user doesn't have
    /// read privileges for it.
    /// 
    /// # Examples
    /// 
    /// See the [`module documentation`](crate::routine).
    pub fn run(&self) -> std::io::Result<()> {
        for item in self.directory.read_dir()? {
            if let Ok(entry) = item {
                if self.pattern.matches(&entry.path()) {
                    fs_utils::remove(&entry.path());
                }
            }
        }
        Ok(())
    } // fn run()
} // impl Routine

/// Spawns a thread that runs a routine repeatedly.
pub fn spawn_routine(routine: Routine) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            routine.run();

            thread::sleep(routine.interval.unsigned_abs());
        }
    })
}
