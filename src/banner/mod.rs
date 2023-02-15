use std::{fs, io, path::PathBuf};

/// Returns the banner string if present
pub fn get() -> Option<io::Result<String>> {
    let path = PathBuf::from("banner.txt");
    if path.exists() {
        Some(fs::read_to_string(&path))
    } else {
        None
    }
}
