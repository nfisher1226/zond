pub mod index;
mod time;

use {
    atom_syndication as atom,
    crate::{
        config::Config,
        traits::ToDisk,
    },
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
        env,
        error::Error,
        fs,
        path::{
            Path,
            PathBuf
        },
        process::Command,
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
    fn publish(&mut self) {
        self.published = Some(Time::now());
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
            path.set_extension("gmi");
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

    pub fn get_path(title: &str, kind: Kind) -> PathBuf {
        let mut tpath = title.trim().to_lowercase().replace(" ", "_");
        tpath.push_str(".gmi");
        let file = match kind {
            Kind::Page(Some(path)) => path,
            Kind::Page(None) => {
                let mut path = PathBuf::from("content");
                path.push(Path::new(&tpath));
                path
            },
            Kind::Post => {
                let mut path = PathBuf::from("content");
                path.push("gemlog");
                path.push(Path::new(&tpath));
                path
            },
        };
        file
    }

    pub fn atom(&self, kind: Kind, config: &Config) -> Result<atom::Entry, Box<dyn Error>> {
        let mut url: Url = format!("gemini://{}", config.domain).parse()?;
        let mut path = PathBuf::from(&config.path.as_ref().unwrap_or(&"/".to_string()));
        let rpath = Self::get_path(&self.title, kind);
        path.push(&rpath);
        url.set_path(&path.to_string_lossy());
        let author = config.author.to_atom();
        let entry = atom::EntryBuilder::default()
            .title(self.title.clone())
            .id(url.to_string())
            .updated(self.published.as_ref().unwrap().to_date_time()?)
            .authors(vec![author])
            .categories(self.categories(&config)?)
            .published(self.published.as_ref().unwrap().to_date_time()?)
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                self.published.as_ref().unwrap().year,
                &config.author.name
            )))
            .summary(self.summary.as_ref().map(|t| atom::Text::plain(t)))
            .build();
        Ok(entry)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    pub meta: Meta,
    pub content: String,
}

impl ToDisk for Page {
    type Err = Box<dyn Error>;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        let mut contents = to_string_pretty(&self.meta, PrettyConfig::new())?;
        contents.push_str("\n---\n");
        contents.push_str(&self.content);
        fs::write(path, contents)?;
        Ok(())
    }
}

impl Page {
    pub fn from_path(file: &PathBuf) -> Option<Self> {
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

    pub fn create(
        kind: Kind,
        title: &str,
        summary: Option<&str>,
        tags: Vec<String>
    ) -> Result<PathBuf, Box<dyn Error>> {
        let mut tpath = title.trim().to_lowercase().replace(" ", "_");
        tpath.push_str(".gmi");
        let file = match kind {
            Kind::Page(Some(path)) => path,
            Kind::Page(None) => {
                let mut path = PathBuf::from("content");
                path.push(&tpath);
                path
            },
            Kind::Post => {
                let mut path = PathBuf::from("content");
                path.push("gemlog");
                path.push(&tpath);
                path
            },
        };
        let parent = file.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
        }
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
        page.to_disk(&file)?;
        Ok(file)
    }

    pub fn publish(kind: Kind, title: &str) -> Result<(), Box<dyn Error>> {
        let path = Meta::get_path(title, kind);
        if let Some(mut page) = Self::from_path(&path) {
            page.meta.publish();
            page.to_disk(&path)?;
        }
        Ok(())
    }

    pub fn edit(kind: Kind, title: &str) -> Result<(), Box<dyn Error>> {
        let path = Meta::get_path(title, kind);
        match env::var("EDITOR") {
            Ok(ed) => {
                Command::new(ed)
                    .arg(&format!("{}", path.display()))
                    .status()?;
            },
            Err(_) => mime_open::open(&format!("{}", path.display()))?,
        }
        Ok(())
    }

    pub fn render(&self, cfg: &Config, path: &Path, depth: usize) -> Result<(), Box<dyn Error>> {
        let mut page = format!(
            "# {}\n### {}\n{}\n\n",
            self.meta.title,
            self.meta.published.as_ref().unwrap().date_string(),
            self.content
        );
        page.push_str(&format!(
            "=> {} Home\n",
            match depth {
                1 => ".".to_string(),
                2 => "..".to_string(),
                _ => cfg.url()?.to_string(),
            }
        ));
        if let Some(p) = path.parent() {
            if let Some(n) = p.file_name() {
                if let Some(s) = n.to_str() {
                    if s == "gemlog" {
                        page.push_str("=> . All posts\n");
                    }
                }
            }
        }
        if let Some(license) = &cfg.license {
            page.push_str(&format!(
                "All content for this site is released under the {} license.\n",
                license.to_string(),
            ));
        }
        page.push_str(&format!(
            "© {} by {}\n",
            self.meta.published.as_ref().unwrap().year,
            cfg.author.name,
        ));
        if cfg.show_email {
            if let Some(ref email) = cfg.author.email {
                page.push_str(&format!(
                    "=> mailto:{} Contact\n",
                    email,
                ));
            }
        }
        if let Some(p) = path.parent() {
            if !p.exists() {
                fs::create_dir_all(p)?;
            }
        }
        fs::write(path, &page)?;
        Ok(())
    }
}
