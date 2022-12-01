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
/// This function returns an error if some external circumstance prevents
/// the file or directory from being removed, for example if the user lacks
/// the permissions to remove it. However, no error is returned if `path`
/// doesn't exist, because that's precisely what someone calling this function
/// should be trying to achieve.
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
    match remove_dir(path) {
        // if path isn't a directory, it must be a file
        Err(e) if error::not_a_directory(e.io_source())
        => remove_file(path),

        other
        => other
    }
}

/// Removes a directory.
/// 
/// One half of [`remove`]. The directory does not need to be empty.
/// 
/// # Errors
/// 
/// See [`remove`] as well as [`std::fs::remove_dir_all`], which is what this
/// function currently uses internally.
/// 
/// # Examples
/// 
/// See [`remove`].
fn remove_dir<P: AsRef<Path>>(path: P) -> Result<(), FailedToRemove> {
    remove_with(
        |p| {fs::remove_dir_all(p)},
        &path
    )
}

/// Removes a file.
/// 
/// One half of [`remove`].
/// 
/// # Errors
/// 
/// See [`remove`] as well as [`std::fs::remove_file`], which is what this
/// function currently uses internally.
/// 
/// # Examples
/// 
/// See [`remove`].
fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), FailedToRemove> {
    remove_with(
        |p| {fs::remove_file(p)},
        &path
    )
}

/// Removes a file or directory using the provided function.
/// 
/// Currently, this function doesn't enforce anything about the internal
/// behaviour of `remover`, because I have no idea how to do that. However,
/// any function that's passed into this function should be expected to try
/// to remove a file or directory, because this function ignores errors
/// resulting from `path` not existing.
/// 
/// # Errors
/// 
/// This function returns any error returned by `remover` unless the error
/// signifies that `path` doesn't exist. See [`remove`].
/// 
/// # Examples
/// 
/// It would be nice if the function could just accept functions such as
/// [`std::fs::remove_file`] as is, but Rust doesn't like that and I have
/// no idea how to get around it without closures.
/// 
/// ```no_run
/// remove_with(
///     |p| {fs::remove_file(p)},
///     r"C:\path\to\file"
/// )
/// ```
fn remove_with<P: AsRef<Path>>(
    remover: fn(&P) -> std::io::Result<()>,
    path: &P
) -> Result<(), FailedToRemove> {
    match remover(path) {
        Ok(_) => Ok(()),

        // we want to remove the path anyway so it not existing is OK
        Err(e) if error::not_found(&e) => Ok(()),

        Err(e) => Err(FailedToRemove::new(path.as_ref(), e))
    }
}
