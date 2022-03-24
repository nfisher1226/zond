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
        io,
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
    pub name: String,
    pub email: Option<String>,
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

impl Default for Feed {
    fn default() -> Self {
        Self::Atom
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Config {
    pub title: String,
    pub author: Person,
    pub domain: String,
    pub path: Option<String>,
    pub entries: u8,
    pub feed: Option<Feed>,
    pub license: Option<License>,
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
}
