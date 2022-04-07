use {
    atom_syndication::Feed,
    std::{
        error::Error,
        io::Write,
        path::{Path, PathBuf},
        process::Stdio,
    },
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
        if let Ok(mut child) = std::process::Command::new("xmllint")
            .args(["-", "--pretty", "1"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            if let Err(e) = child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(self.to_string().as_bytes()) {
                eprintln!("Error writing atom feed to xmllint stdin");
                eprintln!("  Error occured in trait `ToDisk` for `atom_syndication::Feed`");
                return Err(e.into());
            }
            let output = match child.wait_with_output() {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("Error getting child process output");
                    eprintln!("  Error occurred in trait `ToDisk` for `atom_syndication::Feed`");
                    return Err(e.into());
                }
            };
            let atom = String::from_utf8_lossy(&output.stdout);
            match std::fs::write(path, &String::from(atom)) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Error writing atom feed to disk");
                    Err(e.into())
                }
            }
        } else {
            let atom = self.to_string();
            let atom = atom.replace('>', ">\n");
            match std::fs::write(path, &atom) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Error writing atom feed to disk");
                    Err(e.into())
                }
            }
        }
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
