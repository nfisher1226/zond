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
}

impl Config {
    pub fn wizard() -> Result<Self, Box<dyn Error>> {
        let mut title = String::new();
        eprintln!("Title for this capsule:");
        io::stdin().read_line(&mut title)?;
        let title = title.trim_end().to_string();
        let mut name = String::new();
        eprintln!("Author for this capsule:");
        io::stdin().read_line(&mut name)?;
        let name = name.trim_end().to_string();
        let mut email = String::new();
        eprintln!("Author's email (optional):");
        io::stdin().read_line(&mut email)?;
        let email = match email.as_str() {
            s if s.split_once('@').is_some() => Some(s.trim_end().to_string()),
            _ => None,
        };
        let mut url = String::new();
        eprintln!("Author's homepage (optional):");
        io::stdin().read_line(&mut url)?;
        let url = match url.as_str() {
            s if s.split_once('@').is_some() => Some(s.trim_end().to_string()),
            _ => None,
        };
        let author = Person {
            name,
            email,
            url,
        };
        let mut domain = String::new();
        eprintln!("Domain which will serve this capsule: ");
        io::stdin().read_line(&mut domain)?;
        let domain = domain.trim_end().to_string();
        let mut path = String::new();
        eprintln!("Path from the server root to this capsule: ");
        io::stdin().read_line(&mut path)?;
        let path = if path.as_str() == "\n" {
            None
        } else {
            Some(path.trim_end().to_string())
        };
        let mut entries = String::new();
        eprintln!("Number of posts to display links for on the index page:");
        io::stdin().read_line(&mut entries)?;
        let entries: u8 = entries.trim_end().parse()?;
        let mut feed = String::new();
        eprintln!("Type of feed to generate - 'atom', 'gemini', or 'both', or blank for none");
        io::stdin().read_line(&mut feed)?;
        let feed = match feed.as_str() {
            "atom\n" => Some(Feed::Atom),
            "gemini\n" => Some(Feed::Gemini),
            "both\n" => Some(Feed::Both),
            _ => None,
        };
        Ok(Self {
            title,
            author,
            domain,
            path,
            entries,
            feed,
        })
    }

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
