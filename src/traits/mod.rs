use {
    atom_syndication::Feed,
    std::{
        error::Error,
        io::{BufReader, Write},
        fs::File,
        path::{Path, PathBuf},
    },
    xml::{EmitterConfig, EventReader}
};

/// Saves a content type to disk
pub trait ToDisk {
    type Err;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err>;
}

/// Gets the path of a content item
pub trait GetPath {
    fn get_path(root: &Path, subdir: Option<&Path>) -> PathBuf;
}

impl ToDisk for Feed {
    type Err = Box<dyn Error>;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        if let Some(p) = path.parent() {
            if !p.exists() {
                if let Err(e) = std::fs::create_dir_all(&p) {
                    eprintln!("Error creating directory in trait `ToDisk` for `atom_syndication::Feed`");
                    return Err(e.into());
                }
            }
        }
        let ir = self.to_string();
        let reader = BufReader::new(ir.as_bytes());
        let parser = EventReader::new(reader);
        let mut outfd = File::create(path)?;
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut outfd);
        parser.into_iter().for_each(|e| {
            if let Ok(e) = e {
                e.as_writer_event().map(|x| writer.write(x));
            }
        });
        outfd.write_all(b"\n")?;
        Ok(())
    }
}

impl GetPath for Feed {
    fn get_path(root: &Path, _subdir: Option<&Path>) -> PathBuf {
        let mut path = root.to_path_buf();
        path.push("gemlog");
        path.push("atom.xml");
        path
    }
}
