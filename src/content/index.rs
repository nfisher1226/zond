use {
    crate::traits::{
        GetPath,
        ToDisk,
    },
    std::{
        error::Error,
        path::{
            Path,
            PathBuf,
        },
    },
};

/// A wrapper type representing the content of an index page
pub struct Index(pub String);

impl GetPath for Index {
    fn get_path(root: &Path, subdir: Option<&Path>) -> PathBuf {
        let mut idx = root.to_path_buf();
        if let Some(p) = subdir {
            idx.push(p);
        }
        idx.push(PathBuf::from("index.gmi"));
        idx
    }
}

impl ToDisk for Index {
    type Err = Box<dyn Error>;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        std::fs::write(path, &self.0)?;
        Ok(())
    }
}
