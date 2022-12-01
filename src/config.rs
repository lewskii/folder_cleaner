use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use time::Duration;

use crate::routine::Routine;
use crate::fs_utils::FilePattern;


#[derive(Serialize, Deserialize)]
pub struct Config {
    routines: std::vec::Vec<Routine>
}


pub fn routines() -> Vec<Routine> {
    vec![Routine {
        directory: PathBuf::from(r"C:\Users\lewski\Desktop\test"),
        interval: Duration::MINUTE.unsigned_abs(),
        pattern: FilePattern::Any
    }]
}
