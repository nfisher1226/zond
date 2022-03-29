use {
    atom_syndication as atom,
    ron::ser::{
        to_string_pretty,
        PrettyConfig
    },
    serde::{
        Deserialize,
        Serialize
    },
    std::{
        error::Error,
        fmt::Display,
        fs,
        path::PathBuf
    },
    url::Url,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Feed {
    Atom,
    Gemini,
    Both,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum License {
    CcBy,
    CcBySa,
    CcByNc,
    CcByNcSa,
    CcByNd,
    CcByNcNd,
    Other(String),
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::CcBy => "CC BY",
            Self::CcBySa => "CC BY-SA",
            Self::CcByNc => "CC BY-NC",
            Self::CcByNcSa => "CC BY-NC-SA",
            Self::CcByNd => "CC BY-ND",
            Self::CcByNcNd => "CC BY-NC-ND",
            Self::Other(s) => s,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Person {
    /// The author's name
    pub name: String,
    /// Author's email
    pub email: Option<String>,
    /// Author's homepage
    pub url: Option<String>,
}

impl Person {
    pub fn into_atom(self) -> atom::Person {
        atom::Person {
            name: self.name.clone(),
            email: self.email.clone(),
            uri: self.url.clone(),
        }
    }
}

impl Default for Feed {
    fn default() -> Self {
        Self::Atom
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let cfg_file = PathBuf::from("Config.ron");
        let cfg_file = fs::read_to_string(cfg_file).unwrap();
        match ron::de::from_str(&cfg_file) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let ron_str = to_string_pretty(&self, PrettyConfig::new())?;
        fs::write(&PathBuf::from("Config.ron"), ron_str)?;
        Ok(())
    }

    pub fn url(&self) -> Result<Url, Box<dyn Error>> {
        let mut path = PathBuf::new();
        if let Some(p) = &self.path {
            path.push(p);
        }
        let mut url = Url::parse(&format!("gemini://{}", &self.domain))?;
        url.set_path(&format!("{}", path.display()));
        Ok(url)
    }
}
