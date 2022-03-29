use {
    atom_syndication as atom,
    atom_syndication::Feed,
    chrono::{
        Datelike,
        Utc,
    },
    clap::ArgMatches,
    crate::{
        config::Config,
        content::{
            index::Index,
            Kind,
            Meta,
            Page,
        },
        traits::{
            ToDisk,
            GetPath,
        },
    },
    std::{
        collections::{
            BTreeMap,
            HashMap,
        },
        error::Error,
        path::{
            Path,
            PathBuf,
        },
    },
    url::Url,
    walkdir::WalkDir,
};

#[derive(Clone)]
pub struct Link {
    pub url: String,
    pub display: String,
}

impl Link {
    pub fn to_gmi(&self) -> String {
        format!("=> {} {}\n", &self.url, &self.display)
    }

    pub fn get(origin: &Path, cfg: &Config, meta: &Meta) -> Result<Self, Box<dyn Error>> {
        let mut url = cfg.url()?;
        url.set_path(&origin.to_string_lossy());
        Ok(Self {
            url: url.to_string(),
            display: format!(
                "{} - {}",
                meta.published.as_ref().unwrap().date_string(),
                &meta.title,
            )
        })
    }
}

#[derive(Clone)]
struct GemFeed(String);

impl ToDisk for GemFeed {
    type Err = Box<dyn Error>;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        std::fs::write(path, &self.0)?;
        Ok(())
    }
}

impl GetPath for GemFeed {
    fn get_path(root: &mut PathBuf, _subdir: Option<&Path>) -> PathBuf {
        root.push("gemlog");
        root.push("feed.gmi");
        root.to_path_buf()
    }
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let cfg = Config::load()?;
    let mut output = PathBuf::from(matches.value_of("output").unwrap_or("public"));
    if let Some(ref path) = cfg.path {
        output.push(&path);
    }
    if !output.exists() {
        std::fs::create_dir_all(&output)?;
    }
    let mut output = std::fs::canonicalize(&output)?;
    if output.exists() {
        std::fs::remove_dir_all(&output)?;
    }
    let capsule = Capsule::init(&cfg, output.clone())?;
    match &cfg.feed {
        Some(crate::config::Feed::Atom) => {
            let atom = capsule.atom(&cfg)?;
            let dest = Feed::get_path(&mut output, None);
            atom.to_disk(&dest)?;
        },
        Some(crate::config::Feed::Gemini) => {
            let feed = capsule.gemfeed(&cfg)?;
            let dest = GemFeed::get_path(&mut output, None);
            feed.to_disk(&dest)?;
        },
        Some(crate::config::Feed::Both) => {
            let atom = capsule.atom(&cfg)?;
            let dest = Feed::get_path(&mut output.clone(), None);
            atom.to_disk(&dest)?;
            let feed = capsule.gemfeed(&cfg)?;
            let dest = GemFeed::get_path(&mut output.clone(), None);
            feed.to_disk(&dest)?;
        },
        None => {},
    }
    capsule.render_tags(&cfg, &output)?;
    capsule.render_index(&cfg, &output)?;
    capsule.render_gemlog_index(&cfg, &output)?;
    Ok(())
}

fn items(cfg: &Config, output: &PathBuf) -> Result<(BTreeMap<i64, Meta>, HashMap<PathBuf, Meta>, HashMap<String, Vec<Link>>), Box<dyn Error>> {
    let mut posts: BTreeMap<i64, Meta> = BTreeMap::new();
    let mut pages: HashMap<PathBuf, Meta> = HashMap::new();
    let mut tags: HashMap<String, Vec<Link>> = HashMap::new();
    let mut current = std::env::current_dir()?;
    current.push("content");
    let current = std::fs::canonicalize(&current)?;
    let mut index = current.clone();
    index.push("index.gmi");
    let mut gemlog_index = current.clone();
    gemlog_index.push("gemlog");
    gemlog_index.push("index.gmi");
    for entry in WalkDir::new("content") {
        if let Ok(entry) = entry {
            let path = PathBuf::from(entry.path());
            let path = std::fs::canonicalize(&path)?;
            let last = path.strip_prefix(&current)?;
            if let Some(n) = last.to_str() {
                if n == "index.gmi" || n == "gemlog/index.gmi" {
                    continue;
                }
            }
            let mut output = output.clone();
            output.push(&last);
            if let Some(s) = path.extension() {
                if let Some("gmi") = s.to_str() {
                    if let Some(page) = Page::from_path(&path) {
                        if let Some(ref time) = page.meta.published {
                            if &path != &index && &path != &gemlog_index {
                                let depth = entry.depth();
                                let link = Link::get(&path, cfg, &page.meta)?;
                                for tag in &page.meta.tags {
                                    if let Some(t) = tags.get_mut(tag) {
                                        t.push(link.clone());
                                    } else {
                                        tags.insert(tag.to_string(), vec![link.clone()]);
                                    }
                                }
                                if last.starts_with("gemlog") {
                                    page.render(cfg, &output, depth)?;
                                    posts.insert(time.timestamp()?, page.meta);
                                } else {
                                    page.render(cfg, &output, depth)?;
                                    pages.insert(last.to_path_buf(), page.meta);
                                }
                            }
                        }
                    }
                } else {
                    if entry.file_type().is_file() {
                        std::fs::copy(&path, &output)?;
                    }
                }
            } else {
                if entry.file_type().is_file() {
                    std::fs::copy(&path, &output)?;
                }
            }
        }
    }
    Ok((posts, pages, tags))
}

struct Capsule {
    posts: BTreeMap<i64, Meta>,
    pages: HashMap<PathBuf, Meta>,
    tags: HashMap<String, Vec<Link>>,
}

impl Capsule {
    fn init(cfg: &Config, path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let (posts, pages, tags) = items(cfg, &path)?;
        Ok(Self {
            posts,
            pages,
            tags,
        })
    }

    fn atom(&self, cfg: &Config) -> Result<atom::Feed, Box<dyn Error>> {
        let mut entries: Vec<atom::Entry> = vec![];
        for entry in self.posts.values().rev() {
            entries.push(entry.atom(Kind::Post, cfg)?);
        }
        let year = self.posts
            .values()
            .last()
            .unwrap()
            .published
            .as_ref()
            .unwrap()
            .year;
        let mut url = Url::parse(&format!("gemini://{}", &cfg.domain))?;
        if let Some(p) = &cfg.path {
            url.set_path(&p);
        }
        let feed = atom::FeedBuilder::default()
            .title(cfg.title.to_string())
            .id(url.to_string())
            .author(cfg.author.to_atom())
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                year,
                &cfg.author.name
            )))
            .base(url.to_string())
            .entries(entries)
            .build();
        Ok(feed)
    }

    fn gemfeed(&self, cfg: &Config) -> Result<GemFeed, Box<dyn Error>> {
        let mut page = format!("# {}\n\n", &cfg.title);
        for entry in self.posts.values().rev() {
            let mut url: Url = format!("gemini://{}", cfg.domain).parse()?;
            let mut path = PathBuf::from(&cfg.path.as_ref().unwrap_or(&"/".to_string()));
            let rpath = Meta::get_path(&entry.title, Kind::Post);
            path.push(&rpath);
            url.set_path(&path.to_string_lossy());
            page.push_str(&format!(
                "=> {} {} - {}\n",
                url.to_string(),
                entry.published.as_ref().unwrap().date_string(),
                &entry.title,
            ));
        }
        Ok(GemFeed(page))
    }

    fn render_tags(&self, cfg: &Config, output: &Path) -> Result<(), Box<dyn Error>> {
        let index_path = Index::get_path(&mut output.to_path_buf(), Some(&PathBuf::from("tags")));
        let mut index_page = format!("# {}\n\n### All tags\n", &cfg.title);
        for (tag, links) in &self.tags {
            index_page.push_str(&format!("=> {}.gmi {}\n", &tag, &tag));
            let mut dest = PathBuf::from(output);
            dest.push(&PathBuf::from("tags"));
            if !dest.exists() {
                std::fs::create_dir_all(&dest)?;
            }
            dest.push(&PathBuf::from(&tag));
            dest.set_extension("gmi");
            let mut page = format!("# {}\n\n### Pages tagged {}\n", &cfg.title, &tag);
            for link in links {
                page.push_str(&link.to_gmi());
            }
            page.push_str("\n=> . All tags\n");
            page.push_str("=> .. Home\n");
            if let Some(ref license) = cfg.license {
                page.push_str(&format!(
                    "All content for this site is release under the {} license.\n",
                    license.to_string(),
                ));
            }
            page.push_str(&format!("© {} by {}\n", Utc::now().date().year(), &cfg.author.name));
            if cfg.show_email {
                if let Some(ref email) = cfg.author.email {
                    page.push_str(&format!(
                        "=> mailto:{} Contact\n",
                        email,
                    ));
                }
            }
            std::fs::write(&dest, &page.as_bytes())?;
        }
        index_page.push_str("\n=> .. Home\n");
        if let Some(ref license) = cfg.license {
            index_page.push_str(&format!(
                "All content for this site is release under the {} license.\n",
                license.to_string(),
            ));
        }
        index_page.push_str(&format!("© {} by {}\n", Utc::now().date().year(), &cfg.author.name));
        if cfg.show_email {
            if let Some(ref email) = cfg.author.email {
                index_page.push_str(&format!(
                    "=> mailto:{} Contact\n",
                    email,
                ));
            }
        }
        Index(index_page).to_disk(&index_path)?;
        Ok(())
    }

    fn render_index(&self, cfg: &Config, output: &Path) -> Result<(), Box<dyn Error>> {
        let origin = PathBuf::from("content/index.gmi");
        if let Some(page) = Page::from_path(&origin) {
            let mut content = format!("# {}\n\n", &cfg.title);
            content.push_str(&page.content);
            let mut posts = String::from("### Gemlog posts\n");
            let num = std::cmp::min(cfg.entries, self.posts.len());
            for post in self.posts.values().rev().take(num) {
                let path = Meta::get_path(&post.title, Kind::Post);
                posts.push_str(&format!(
                    "=> gemlog/{} {} - {}\n",
                    path.file_name().unwrap().to_str().unwrap(),
                    post.published.as_ref().unwrap().date_string(),
                    &post.title,
                ));
            }
            posts.push_str("=> gemlog/ All posts");
            let mut content = content.replace("{% posts %}", &posts);
            // TODO - index these
            for (_path,_page) in &self.pages {
            }
            if let Some(ref license) = cfg.license {
                content.push_str(&format!(
                    "\n\nAll content for this site is release under the {} license.\n",
                    license.to_string(),
                ));
            }
            content.push_str(&format!("© {} by {}\n", Utc::now().date().year(), &cfg.author.name));
            if cfg.show_email {
                if let Some(ref email) = cfg.author.email {
                    content.push_str(&format!(
                        "=> mailto:{} Contact\n",
                        email,
                    ));
                }
            }
            let path = Index::get_path(&mut PathBuf::from(output), None);
            Index(content).to_disk(&path)?;
        }
        Ok(())
    }

    fn render_gemlog_index(&self, cfg: &Config, output: &Path) -> Result<(), Box<dyn Error>> {
        let origin = PathBuf::from("content/gemlog/index.gmi");
        if let Some(page) = Page::from_path(&origin) {
            let mut content = format!("# {}\n\n", &cfg.title);
            content.push_str(&page.content);
            content.push_str("\n\n### Gemlog posts\n");
            for post in self.posts.values().rev() {
                let path = Meta::get_path(&post.title, Kind::Post);
                content.push_str(&format!(
                    "=> {} {} - {}\n",
                    path.file_name().unwrap().to_str().unwrap(),
                    post.published.as_ref().unwrap().date_string(),
                    &post.title,
                ));
            }
            content.push_str("\n=> ../tags tags\n=> .. Home\n");
            if let Some(ref license) = cfg.license {
                content.push_str(&format!(
                    "All content for this site is release under the {} license.\n",
                    license.to_string(),
                ));
            }
            content.push_str(&format!("© {} by {}\n", Utc::now().date().year(), &cfg.author.name));
            if cfg.show_email {
                if let Some(ref email) = cfg.author.email {
                    content.push_str(&format!(
                        "=> mailto:{} Contact\n",
                        email,
                    ));
                }
            }
            let path = Index::get_path(&mut PathBuf::from(output), Some(&PathBuf::from("gemlog")));
            Index(content).to_disk(&path)?;
        }
        Ok(())
    }
}
