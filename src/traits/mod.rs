use {
    atom_syndication::Feed,
    std::{
        error::Error,
        io::Write,
        path::{
            Path,
            PathBuf,
        },
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
                std::fs::create_dir_all(&p)?;
            }
        }
        match std::process::Command::new("xmllint")
            .arg("-")
            .arg("--pretty")
            .arg("1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Ok(mut child) => {
                child.stdin.as_mut().unwrap().write_all(self.to_string().as_bytes())?;
                let output = child.wait_with_output()?;
                let atom = String::from_utf8_lossy(&output.stdout);
                std::fs::write(path, &String::from(atom))?;
            },
            Err(_) => {
                let atom = self.to_string();
                let atom = atom.replace(">", ">\n");
                std::fs::write(path, &atom)?;
            },
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
