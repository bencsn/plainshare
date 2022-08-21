use std::io;
use std::fs::{self};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(
    dir: &Path, 
    visited_paths: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, visited_paths)?;
            } else {
                println!("Visiting.... {:?}", path); 
                visited_paths.push(path.to_str().unwrap().to_string());
            }
        }
    }
    Ok(())
}