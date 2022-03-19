mod time;

use {
    atom_syndication as atom,
    crate::config::Config,
    extract_frontmatter::Extractor,
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
        path::{
            Path,
            PathBuf
        }
    },
    time::Time,
    url::Url,
};

#[derive(Clone, Debug)]
pub enum Kind {
    Page(Option<PathBuf>),
    Post,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub title: String,
    pub summary: Option<String>,
    pub published: Option<Time>,
    pub tags: Vec<String>,
}

impl Meta {
    fn new(title: &str, summary: Option<&str>, tags: Vec<String>) -> Self {
        Self {
            title: title.to_string(),
            summary: summary.map(|x| x.to_string()),
            published: None,
            tags,
        }
    }

    fn publish(&mut self) {
        self.published = Some(Time::create());
    }

    fn categories(&self, cfg: &Config) -> Result<Vec<atom::Category>, Box<dyn Error>> {
        let mut categories = Vec::new();
        for tag in &self.tags {
            let mut url = Url::parse(&format!("gemini://{}", cfg.domain))?;
            let mut path = match &cfg.path {
                Some(p) => PathBuf::from(&p),
                None => PathBuf::from("/"),
            };
            path.push(&PathBuf::from("tags"));
            path.push(&PathBuf::from(&tag));
            path.set_extension(".gmi");
            let path = path.to_string_lossy();
            url.set_path(&path);
            let cat = atom::Category {
                term: tag.clone(),
                scheme: Some(url.to_string()),
                label: Some(tag.clone()),
            };
            categories.push(cat);
        }
        Ok(categories)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    pub meta: Meta,
    pub content: String,
}

impl Page {
    fn get_path(title: &str, kind: Kind) -> PathBuf {
        let tpath = title.trim().to_lowercase().replace(" ", "_");
        let mut file = PathBuf::from("content");
        match kind {
            Kind::Page(path) => {
                if let Some(p) = path {
                    file.push(&p);
                }
            },
            Kind::Post => file.push("gemlog"),
        }
        file.push(Path::new(&tpath));
        file.set_extension("gmi");
        file
    }

    fn from_path(file: &PathBuf) -> Option<Self> {
        match fs::read_to_string(file) {
            Ok(f) => {
                let mut extractor = Extractor::new(&f);
                extractor.select_by_terminator("---");
                let (fm,doc): (Vec<&str>, &str) = extractor.split();
                let fm = fm.join("\n");
                let content = doc.trim().to_string();
                match ron::de::from_str(&fm) {
                    Ok(meta) => Some(Self {
                        meta,
                        content,
                    }),
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    fn get(title: &str, kind: Kind) -> Option<Self> {
        let file = Self::get_path(title, kind);
        Self::from_path(&file)
    }

    fn save(&self, file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut contents = to_string_pretty(&self.meta, PrettyConfig::new())?;
        contents.push_str("\n---\n");
        contents.push_str(&self.content);
        fs::write(file, contents)?;
        Ok(())
    }

    pub fn create(
        kind: Kind,
        title: &str,
        summary: Option<&str>,
        tags: Vec<String>
    ) -> Result<PathBuf, Box<dyn Error>> {
        let mut tpath = title.trim().to_lowercase().replace(" ", "_");
        tpath.push_str(".gmi");
        let mut file = PathBuf::from("content");
        match kind {
            Kind::Page(path) => {
                if let Some(p) = path {
                    file.push(&p);
                }
            },
            Kind::Post => file.push("gemlog"),
        };
        if !file.exists() {
            fs::create_dir_all(&file)?;
        }
        file.push(Path::new(&tpath));
        let meta = Meta {
            title: title.to_string(),
            summary: summary.map(|x| x.to_string()),
            published: None,
            tags,
        };
        let page = Self {
            meta,
            content: String::new(),
        };
        page.save(&file)?;
        Ok(file)
    }

    pub fn publish(kind: Kind, title: &str) -> Result<(), Box<dyn Error>> {
        let path = Self::get_path(title, kind);
        if let Some(mut page) = Self::from_path(&path) {
            page.meta.publish();
            page.save(&path)?;
        }
        Ok(())
    }

    pub fn timestamp(&self) -> Result<i64, Box<dyn Error>> {
        match &self.meta.published {
            Some(time) => Ok(time.timestamp()?),
            None => Err(String::from("Not published").into()),
        }
    }

    fn published(&self) -> bool {
        self.meta.published.is_some()
    }

    pub fn rss_entry(&self, kind: Kind, config: &Config) -> Result<atom::Entry, Box<dyn Error>> {
        let mut url: Url = config.domain.parse()?;
        let mut path = PathBuf::from(&config.path.as_ref().unwrap_or(&"/".to_string()));
        let rpath = Self::get_path(&self.meta.title, kind);
        path.push(&rpath);
        url.set_path(&path.to_string_lossy());
        let author = config.author.to_atom();
        let entry = atom::EntryBuilder::default()
            .title(self.meta.title.clone())
            .id(url.to_string())
            .updated(self.meta.published.as_ref().unwrap().to_date_time()?)
            .authors(vec![author])
            .categories(self.meta.categories(&config)?)
            .published(self.meta.published.as_ref().unwrap().to_date_time()?)
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                self.meta.published.as_ref().unwrap().year,
                &config.author.name
            )))
            .summary(self.meta.summary.as_ref().map(|t| atom::Text::plain(t)))
            .build();
        Ok(entry)
    }
}
