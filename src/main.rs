#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use folder_cleaner::config;
use folder_cleaner::routine;

fn main() {
    let routines = config::routines();
    
    for r in routines {
        let t = routine::spawn_routine(r);
        t.join().unwrap();
    }
}
