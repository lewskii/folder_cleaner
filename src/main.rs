//#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::time::Duration;

use folder_cleaner::routine::Routine;
use folder_cleaner::fs_utils::FilePattern;


fn main() {
    let desktop_routine = Routine {
        dir: PathBuf::from(r"C:\Users\lewski\Desktop\test"),
        interval: Duration::from_secs(60),
        pattern: FilePattern::Extension("lnk".into())
    };

    if let Err(e) = desktop_routine.run() {
        println!("{e}");
    }

}
