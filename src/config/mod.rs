/// Licensing for the content of the capsule
mod license;

pub use license::License;

use crate::Error;
use {
    crate::link::Link,
    atom_syndication as atom,
    ron::ser::{to_writer_pretty, PrettyConfig},
    serde::{Deserialize, Serialize},
    std::{
        fs::{self, File},
        io::BufWriter,
        path::PathBuf,
        str::FromStr,
    },
    url::Url,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// The type of feed to generate
pub enum Feed {
    /// Only an Atom feed will be generated
    #[default]
    Atom,
    /// Only a Gemini feed will be generated
    Gemini,
    /// Both Atom and Gemini feeds will be generated
    Both,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// The primary author of the capsule
pub struct Person {
    /// The author's name
    pub name: String,
    /// Author's email
    pub email: Option<String>,
    /// Author's homepage
    pub url: Option<String>,
}

impl Person {
    pub fn to_atom(&self) -> atom::Person {
        atom::Person {
            name: self.name.clone(),
            email: self.email.clone(),
            uri: self.url.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DisplayDate {
    /// Always display the publication date
    Always,
    /// Only display the publication date on gemlog posts
    #[default]
    GemlogOnly,
    /// Never display the date
    Never,
}

impl FromStr for DisplayDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" | "Always" => Ok(Self::Always),
            "gemlogonly" | "gemlog" | "gemlog_only" | "GemlogOnly" | "Gemlog" => {
                Ok(Self::GemlogOnly)
            }
            "never" | "Never" => Ok(Self::Never),
            _ => Err(Error::ParseEnumError),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Site wide capsule settings
pub struct Config {
    /// Title of the entire capsule
    pub title: String,
    /// Author for the entire capsule
    pub author: Person,
    /// The domain serving the capsule
    pub domain: String,
    /// The path from the server root to the capsule
    pub path: Option<String>,
    /// The number of gemlog entries to display on the main index
    pub entries: usize,
    /// Which pages to display the publication date for
    pub display_date: DisplayDate,
    /// Whether to generate atom and/or gemini feeds
    pub feed: Option<Feed>,
    /// The license which the content of this capsule is published under
    pub license: Option<License>,
    /// Whether to provide a `mailto:` link to the author's email
    pub show_email: bool,
    /// A collection of links to display at the bottom of each page
    pub footer_links: Vec<Link>,
}

impl Config {
    /// Load the config from disk
    pub fn load() -> Result<Self, crate::Error> {
        let cfg_file = PathBuf::from("Config.ron");
        let cfg_file = match fs::read_to_string(cfg_file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading config file");
                return Err(e.into());
            }
        };
        match ron::de::from_str(&cfg_file) {
            Ok(s) => Ok(s),
            Err(e) => {
                eprintln!(
                    "Error decoding config:\n  code: {:?}\n  position:\n    line: {}\n    column: {}",
                    e.code,
                    e.position.line,
                    e.position.col,
                );
                Err(e.into())
            }
        }
    }

    /// Save the config to disk
    pub fn save(&self) -> Result<(), crate::Error> {
        let pcfg = PrettyConfig::new().struct_names(true).decimal_floats(true);
        let buf = File::create("Config.ron")?;
        let writer = BufWriter::new(buf);
        if let Err(e) = to_writer_pretty(writer, &self, pcfg) {
            eprintln!(
                "Error encoding config:\n  code: {:?}\n  position:\n    line: {}\n    column: {}",
                e.code, e.position.line, e.position.col,
            );
            return Err(e.into());
        }
        Ok(())
    }

    /// Returns the address for the root of this capsule
    pub fn url(&self) -> Result<Url, crate::Error> {
        let mut path = PathBuf::new();
        if let Some(p) = &self.path {
            path.push(p);
        }
        let mut url = match Url::parse(&format!("gemini://{}", &self.domain)) {
            Ok(u) => u,
            Err(e) => {
                eprintln!("Error parsing url from config data");
                return Err(e.into());
            }
        };
        url.set_path(&format!("{}", path.display()));
        Ok(url)
    }
}
