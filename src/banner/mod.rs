use std::{fs, io::Error, path::PathBuf};

pub fn get() -> Option<Result<String, Error>> {
    let path = PathBuf::from("banner.txt");
    if path.exists() {
        Some(fs::read_to_string(&path))
    } else {
        None
    }
}
