use {
    atom_syndication as atom,
    atom_syndication::Feed,
    clap::ArgMatches,
    crate::{
        config::Config,
        content::{
            Kind,
            Meta,
            Page,
        },
    },
    std::{
        collections::{
            BTreeMap,
            HashMap,
        },
        error::Error,
        path::PathBuf
    },
    url::Url,
    walkdir::WalkDir,
};

struct Capsule {
    path: PathBuf,
    posts: BTreeMap<i64, Meta>,
    pages: HashMap<PathBuf, Meta>,
}

impl Capsule {
    fn init(cfg: &Config, path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let (posts, pages) = items(cfg, &path)?;
        Ok(Self {
            path,
            posts,
            pages,
        })
    }

    fn gen_rss(&self, cfg: &Config) -> Result<atom::Feed, Box<dyn Error>> {
        let mut entries: Vec<atom::Entry> = vec![];
        for entry in self.posts.values().rev() {
            entries.push(entry.rss_entry(Kind::Post, cfg)?);
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

    fn build(&self, cfg: &Config) -> Result<(), Box<dyn Error>> {
        let rss = self.gen_rss(cfg);
        Ok(())
    }
}


fn posts() -> Result<BTreeMap<i64, Page>, Box<dyn Error>> {
    let mut posts = BTreeMap::new();
    for entry in WalkDir::new("content/gemlog") {
        if let Ok(entry) = entry {
            if let Some(s) = entry.path().extension() {
                if let Some("gmi") = s.to_str() {
                    if let Some(page) = Page::from_path(&PathBuf::from(entry.path())) {
                        if let Some(ref time) = page.meta.published {
                            posts.insert(time.timestamp()?, page);
                        }
                    }
                }
            }
        }
    }
    Ok(posts)
}

fn pages() -> HashMap<PathBuf, Page> {
    let mut pages = HashMap::new();
    for entry in WalkDir::new("content") {
        if let Ok(entry) = entry {
            let path = PathBuf::from(entry.path());
            if !path.starts_with("content/gemlog") {
                if let Some(s) = path.extension() {
                    if let Some("gmi") = s.to_str() {
                        if let Some(page) = Page::from_path(&path) {
                            pages.insert(path, page);
                        }
                    }
                }
            }
        }
    }
    pages
}

fn items(cfg: &Config, output: &PathBuf) -> Result<(BTreeMap<i64, Meta>, HashMap<PathBuf, Meta>), Box<dyn Error>> {
    let mut posts: BTreeMap<i64, Meta> = BTreeMap::new();
    let mut pages: HashMap<PathBuf, Meta> = HashMap::new();
    let mut current = std::env::current_dir()?;
    current.push("content");
    let current = std::fs::canonicalize(&current)?;
    for entry in WalkDir::new("content") {
        if let Ok(entry) = entry {
            let path = PathBuf::from(entry.path());
            let path = std::fs::canonicalize(&path)?;
            let last = path.strip_prefix(&current)?;
            let mut output = output.clone();
            output.push(&last);
            if let Some(parent) = output.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent)?;
                }
            }
            if let Some(s) = path.extension() {
                if let Some("gmi") = s.to_str() {
                    if let Some(page) = Page::from_path(&path) {
                        if let Some(ref time) = page.meta.published {
                            let path = path.strip_prefix(&current)?;
                            if path.starts_with("gemlog") {
                                page.render(&output)?;
                                posts.insert(time.timestamp()?, page.meta);
                            } else {
                                page.render(&output)?;
                                pages.insert(path.to_path_buf(), page.meta);
                            }
                        }
                    }
                } else {
                    std::fs::copy(&path, &output)?;
                }
            } else {
                std::fs::copy(&path, &output)?;
            }
        }
    }
    Err(String::from("Unimplemented").into())
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let cfg = Config::load()?;
    let mut output = PathBuf::from(matches.value_of("output").unwrap_or("public"));
    if let Some(ref path) = cfg.path {
        output.push(&path);
    }
    let output = std::fs::canonicalize(&output)?;
    let capsule = Capsule::init(&cfg, output)?;
    capsule.build(&cfg)?;
    Ok(())
}
