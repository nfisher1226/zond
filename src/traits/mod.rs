use {
    atom_syndication::Feed,
    std::{
        error::Error,
        fs::File,
        io::{BufReader, Write},
        path::{Path, PathBuf},
    },
    xml::{EmitterConfig, EventReader},
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
                    eprintln!(
                        "Error creating directory in trait `ToDisk` for `atom_syndication::Feed`"
                    );
                    return Err(e.into());
                }
            }
        }
        let ir = self.to_string();
        let reader = BufReader::new(ir.as_bytes());
        let parser = EventReader::new(reader);
        let mut outfd = match File::create(path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!(
                    "Error creating file in trait `ToDisk` for `atom_syndication::Feed`"
                );
                return Err(e.into());
            }
        };
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut outfd);
        parser.into_iter().for_each(|e| {
            if let Ok(e) = e {
                e.as_writer_event().map(|x| writer.write(x));
            }
        });
        if let Err(e) = outfd.write_all(b"\n") {
            eprintln!(
                "Error writing to file in trait `ToDisk` for `atom_syndication::Feed`"
            );
            return Err(e.into());
        }
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
