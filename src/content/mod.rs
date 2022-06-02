/// Get an appropriate editor
mod editor;
/// Working with the main and gemlog indexes
pub mod index;
/// Date and time functionality
mod time;

pub use time::Time;
use {
    crate::{config::Config, ToDisk},
    atom_syndication as atom,
    extract_frontmatter::Extractor,
    ron::ser::{to_string_pretty, PrettyConfig},
    serde::{Deserialize, Serialize},
    std::{
        borrow::Cow,
        error::Error,
        fmt::Write,
        fs,
        path::{Path, PathBuf},
    },
    url::Url,
};

#[derive(Clone, Debug)]
/// The content type, page or post
pub enum Kind {
    /// An ordinary page plus the path from the content root
    Page(
        /// The path from the capsule root to this document
        Option<PathBuf>,
    ),
    /// A gemlog post
    Post,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Metadata for a page or post
pub struct Meta {
    /// The title of this page
    pub title: String,
    /// A brief summary of this page which will appear in the atom feed
    pub summary: Option<String>,
    /// If unset, this page will not be included in the generated output. If set,
    /// this will represent the date and time of publication
    pub published: Option<Time>,
    /// Categories for this page
    pub tags: Vec<String>,
}

impl Meta {
    /// Marks this item as published with a publishing time corresponding the
    /// current UTC time
    fn publish(&mut self) {
        self.published = Some(Time::now());
    }

    /// Returns a `Vec` of `atom_syndication::Categories` from the tags of this item
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

    /// Given the title and `Kind` of this item, returns the path to the source file
    pub fn get_path(title: &str, kind: Kind) -> PathBuf {
        let mut tpath = title.trim().to_lowercase().replace(' ', "_");
        tpath.push_str(".gmi");
        match kind {
            Kind::Page(Some(path)) => path,
            Kind::Page(None) => ["content", &tpath].iter().collect(),
            Kind::Post => ["content", "gemlog", &tpath].iter().collect(),
        }
    }

    /// Generates an atom feed entry for this post
    pub fn atom(&self, kind: Kind, config: &Config) -> Result<atom::Entry, Box<dyn Error>> {
        let mut url: Url = format!("gemini://{}", config.domain).parse()?;
        let mut path = PathBuf::from(&config.path.as_ref().unwrap_or(&"/".to_string()));
        let rpath = Self::get_path(&self.title, kind);
        let rpath = rpath.strip_prefix("content")?;
        path.push(&rpath);
        url.set_path(&path.to_string_lossy());
        let url = url.to_string();
        let mut link = atom::Link::default();
        link.set_href(&url);
        link.set_rel("alternate");
        let author = config.author.to_atom();
        let entry = atom::EntryBuilder::default()
            .title(self.title.clone())
            .id(url)
            .updated(self.published.as_ref().unwrap().to_date_time()?)
            .authors(vec![author])
            .categories(self.categories(config)?)
            .link(link)
            .published(self.published.as_ref().unwrap().to_date_time()?)
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                self.published.as_ref().unwrap().year(),
                &config.author.name
            )))
            .summary(self.summary.as_ref().map(atom::Text::plain))
            .build();
        Ok(entry)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// A freestanding page or gemlog post
pub struct Page {
    /// Metadata about this page
    pub meta: Meta,
    /// The content used to generate this page
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
    /// Retreive a `Page` given it's path
    pub fn from_path(file: &Path) -> Option<Self> {
        match fs::read_to_string(file) {
            Ok(f) => {
                let mut extractor = Extractor::new(&f);
                extractor.select_by_terminator("---");
                let (fm, doc): (Vec<&str>, &str) = extractor.split();
                let fm = fm.join("\n");
                let content = doc.trim().to_string();
                match ron::de::from_str(&fm) {
                    Ok(meta) => Some(Self { meta, content }),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Create a new `Page`
    pub fn create(
        kind: Kind,
        title: &str,
        summary: Option<&str>,
        tags: Vec<String>,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let mut tpath = title.trim().to_lowercase().replace(' ', "_");
        tpath.push_str(".gmi");
        let file = match kind {
            Kind::Page(Some(path)) => path,
            Kind::Page(None) => {
                let mut path = PathBuf::from("content");
                path.push(&tpath);
                path
            }
            Kind::Post => {
                let mut path = PathBuf::from("content");
                path.push("gemlog");
                path.push(&tpath);
                path
            }
        };
        let parent = file.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
        }
        let meta = Meta {
            title: title.to_string(),
            summary: summary.map(std::string::ToString::to_string),
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

    /// Publish a page given it's `Kind` and title
    pub fn publish(kind: Kind, title: &str) -> Result<(), Box<dyn Error>> {
        let path = Meta::get_path(title, kind);
        if let Some(mut page) = Self::from_path(&path) {
            page.meta.publish();
            page.to_disk(&path)?;
        }
        Ok(())
    }

    /// Open a `Page` in your editor
    pub fn edit(kind: Kind, title: &str) -> Result<(), Box<dyn Error>> {
        let path = Meta::get_path(title, kind);
        editor::edit(&format!("{}", path.display()))?;
        Ok(())
    }

    /// Render a page and save it to disk
    pub fn render(
        &self,
        cfg: &Config,
        path: &Path,
        depth: usize,
        banner: &Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut page = match banner {
            Some(s) => format!(
                "```\n{s}\n```\n# {}\n### {}\n{}\n\n",
                self.meta.title,
                self.meta.published.as_ref().unwrap().date_string(),
                self.content
            ),
            None => format!(
                "# {}\n### {}\n{}\n\n",
                self.meta.title,
                self.meta.published.as_ref().unwrap().date_string(),
                self.content
            ),
        };
        if !self.meta.tags.is_empty() {
            writeln!(page, "### Tags for this page")?;
            let u = cfg.url()?;
            for tag in &self.meta.tags {
                match depth {
                    1 => writeln!(page, "=> tags/{tag}.gmi {tag}")?,
                    2 => writeln!(page, "=> ../tags/{tag}.gmi {tag}")?,
                    3 => writeln!(page, "=> ../../tags/{tag}.gmi {tag}")?,
                    _ => writeln!(page, "=> {u}/tags/{tag}.gmi {tag}")?,
                }
            }
            page.push('\n');
        }
        writeln!(
            page,
            "=> {} Home",
            match depth {
                1 => Cow::from("."),
                2 => Cow::from(".."),
                _ => Cow::from(cfg.url()?.to_string()),
            }
        )?;
        if let Some(p) = path.parent() {
            if let Some(n) = p.file_name() {
                if let Some(s) = n.to_str() {
                    if s == "gemlog" {
                        writeln!(page, "=> . All posts")?;
                    }
                }
            }
        }
        let year = self.meta.published.as_ref().unwrap().year();
        crate::footer(&mut page, year, cfg)?;
        if let Some(p) = path.parent() {
            if !p.exists() {
                fs::create_dir_all(p)?;
            }
        }
        fs::write(path, &page)?;
        Ok(())
    }
}
