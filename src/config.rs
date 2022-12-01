use std::path::PathBuf;
use time::Duration;

use crate::routine::Routine;
use crate::fs_utils::FilePattern;

pub fn routines() -> Vec<Routine> {
    vec![Routine {
        directory: PathBuf::from(r"C:\Users\lewski\Desktop\test"),
        interval: Duration::MINUTE,
        pattern: FilePattern::Any
    }]
}
