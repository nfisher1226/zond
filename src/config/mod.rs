/// Licensing for the content of the capsule
mod license;
pub use license::License;
use {
    atom_syndication as atom,
    ron::ser::{to_string_pretty, PrettyConfig},
    serde::{Deserialize, Serialize},
    std::{error::Error, fs, path::PathBuf},
    url::Url,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// The type of feed to generate
pub enum Feed {
    /// Only an Atom feed will be generated
    Atom,
    /// Only a Gemini feed will be generated
    Gemini,
    /// Both Atom and Gemini feeds will be generated
    Both,
}

impl Default for Feed {
    fn default() -> Self {
        Self::Atom
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    /// Whether to generate atom and/or gemini feeds
    pub feed: Option<Feed>,
    /// The license which the content of this capsule is published under
    pub license: Option<License>,
    /// Whether to provide a `mailto:` link to the author's email
    pub show_email: bool,
}

impl Config {
    /// Load the config from disk
    pub fn load() -> Result<Self, Box<dyn Error>> {
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
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let ron_str = match to_string_pretty(&self, PrettyConfig::new()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "Error encoding config:\n  code: {:?}\n  position:\n    line: {}\n    column: {}",
                    e.code,
                    e.position.line,
                    e.position.col,
                );
                return Err(e.into());
            }
        };
        match fs::write(&PathBuf::from("Config.ron"), ron_str) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error writing config file");
                Err(e.into())
            }
        }
    }

    /// Returns the address for the root of this capsule
    pub fn url(&self) -> Result<Url, Box<dyn Error>> {
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
