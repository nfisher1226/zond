#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]
use {
    atom_syndication::Feed,
    config::Config,
    gettextrs::gettext,
    once_cell::sync::Lazy,
    std::{
        fs::{self, File},
        io::{BufReader, BufWriter, Write},
        path::{Path, PathBuf},
        process,
    },
    xml::{EmitterConfig, EventReader},
};

/// Adds an ascii banner to each page (if the file banner.txt exists)
pub(crate) mod banner;
/// Generates the command line options struct
pub mod cli;
/// Parses out the subcommands from the cli
pub mod command;
/// Holds the capsule level configuration
pub(crate) mod config;
/// Working with pages and gemlog posts
pub(crate) mod content;
/// Zond errors
pub mod error;
/// A Link
pub(crate) mod link;
/// A gemlog post
pub(crate) mod post;
/// An optional tinylog
pub(crate) mod tinylog;

pub use {content::edit, error::Error};

static CONFIG: Lazy<Config> = Lazy::new(|| match Config::load() {
    Ok(c) => c,
    Err(e) => {
        eprintln!("{}: {e}", gettext("Error loading config"));
        process::exit(1);
    }
});

/// Saves a content type to disk
pub trait ToDisk {
    type Err;

    /// Saves the type to disk
    /// # Errors
    /// Returns error if unable to write to disk
    fn to_disk(&self, path: &Path) -> Result<(), Self::Err>;
}

/// Gets the path of a content item
pub trait GetPath {
    fn get_path(root: &Path, subdir: Option<&Path>) -> PathBuf;
}

impl ToDisk for Feed {
    type Err = Error;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        if let Some(p) = path.parent() {
            if !p.exists() {
                if let Err(e) = fs::create_dir_all(p) {
                    eprintln!("{}",
                        gettext("Error creating directory in trait `ToDisk` for `atom_syndication::Feed`")
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
                    "{}",
                    gettext("Error creating file in trait `ToDisk` for `atom_syndication::Feed`")
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
                "{}",
                gettext("Error writing to file in trait `ToDisk` for `atom_syndication::Feed`")
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

/// Writes the footer for each page
/// # Errors
/// Returns `fmt::Error` if formatting fails
/// Returns `io::Error` if writing to disk fails
pub fn write_footer(writer: &mut BufWriter<File>, year: i32) -> Result<(), crate::Error> {
    writeln!(writer)?;
    if let Some(license) = &CONFIG.license {
        writeln!(
            writer,
            "{} {license}.",
            gettext("All content for this site is licensed as")
        )?;
    }
    writeln!(writer, "© {year} {} {}", gettext("by"), CONFIG.author.name,)?;
    for link in &CONFIG.footer_links {
        writeln!(writer, "{link}")?;
    }
    if CONFIG.show_email {
        if let Some(ref email) = CONFIG.author.email {
            writeln!(writer, "=> mailto:{email} {}", gettext("Contact"))?;
        }
    }
    Ok(())
}
