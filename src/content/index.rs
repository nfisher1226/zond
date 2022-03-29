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

pub struct Index(pub String);

impl GetPath for Index {
    fn get_path(root: &mut PathBuf, subdir: Option<&Path>) -> PathBuf {
        let mut idx = root.clone();
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
