use std::path::PathBuf;
use std::time::Duration;
use std::thread;

use crate::fs_utils::{self, FilePattern};


pub struct Routine {
    pub dir: PathBuf,
    pub interval: Duration,
    pub pattern: FilePattern
}

impl Routine {
    pub fn run(&self) -> std::io::Result<()> {
        for item in self.dir.read_dir()? {
            if let Ok(entry) = item {
                if self.pattern.matches(&entry.path()) {
                    fs_utils::remove(&entry.path());
                }
            }
        }
        Ok(())
    } // fn run()
} // impl Routine

pub fn spawn_routine(r: Routine) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            r.run();

            thread::sleep(r.interval);
        }
    })
}
