use std::fs;
use std::path::Path;
use std::result::Result;

use crate::fs_utils::error::{self, FailedToRemove};


/// Removes a file or directory.
/// 
/// This function acts like a combination of [`std::fs::remove_file`]
/// and [`std::fs::remove_dir_all`], removing the filesystem object at
/// `path` regardless of what it is.
/// 
/// # Errors
/// 
/// This function will return an error if some external circumstance
/// prevents the file or directory from being removed, for example if
/// the user lacks permissions to remove it. However, no error is
/// returned if `path` doesn't point to anything, because that's precisely
/// what someone calling this function should be trying to achieve.
/// 
/// # Examples
/// 
/// ```no_run
/// use folder_cleaner::fs_utils::remove;
/// 
/// let result = remove(r"C:\path\to\dir\or\file");
/// ```
pub fn remove<P: AsRef<Path>>(path: P) -> Result<(), FailedToRemove> {
    let path = path.as_ref();
    match try_remove_dir(path) {
        // if path isn't a directory, it must be a file
        Err(e) if error::not_a_directory(&e.source())
        => try_remove_file(path),

        other
        => other
    }
}

fn try_remove_dir<P: AsRef<Path>>(path: P) -> Result<(), FailedToRemove> {
    try_remove(
        |p| {fs::remove_dir_all(p)},
        path.as_ref()
    )
}

fn try_remove_file<P: AsRef<Path>>(path: P) -> Result<(), FailedToRemove> {
    try_remove(
        |p| {fs::remove_file(p)},
        path.as_ref()
    )
}

fn try_remove<P: AsRef<Path>>(
    remover: fn(&Path) -> std::io::Result<()>,
    path: P
) -> Result<(), FailedToRemove> {
    let path = path.as_ref();
    match remover(path) {
        Ok(_) => Ok(()),

        // we want to remove the path anyway so it not existing is OK
        Err(e) if error::not_found(&e) => Ok(()),

        Err(e) => Err(FailedToRemove::new(path, e))
    }
}
