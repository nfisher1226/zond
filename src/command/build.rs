use {
    crate::{
        config::Config,
        content::{index::Index, Page, Time},
        link::Link,
        post::Post,
        AsAtom, GetPath, ToDisk,
    },
    atom_syndication as atom,
    atom::Feed,
    chrono::{Datelike, Utc},
    clap::ArgMatches,
    std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
        fmt::Write,
        fs,
        path::{Path, PathBuf},
    },
    url::Url,
    walkdir::WalkDir,
};

/// A `BTreeMap` of gemlog posts
type Posts = BTreeMap<i64, Post>;
/// A `HashMap` of tag names and their associated links
type Tags = HashMap<String, Vec<Link>>;

#[derive(Clone)]
/// Wrapper type around the text of a gemini feed
struct GemFeed(String);

impl ToDisk for GemFeed {
    type Err = crate::Error;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        fs::write(path, &self.0)?;
        Ok(())
    }
}

impl GetPath for GemFeed {
    fn get_path(root: &Path, _subdir: Option<&Path>) -> PathBuf {
        let mut path = root.to_path_buf();
        path.push("gemlog");
        path.push("feed.gmi");
        path
    }
}

/// Performs the build
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let cfg = Config::load()?;
    let mut output = PathBuf::from(matches.value_of("output").unwrap_or("public"));
    if let Some(ref path) = cfg.path {
        output.push(&path);
    }
    if !output.exists() {
        std::fs::create_dir_all(&output)?;
    }
    let output = std::fs::canonicalize(&output)?;
    if output.exists() {
        std::fs::remove_dir_all(&output)?;
    }
    let capsule = Capsule::init(&cfg, &output)?;
    match &cfg.feed {
        Some(crate::config::Feed::Atom) => {
            let atom = capsule.as_atom(&cfg)?;
            let dest = Feed::get_path(&output, None);
            atom.to_disk(&dest)?;
        }
        Some(crate::config::Feed::Gemini) => {
            let feed = capsule.gemfeed(&cfg)?;
            let dest = GemFeed::get_path(&output, None);
            feed.to_disk(&dest)?;
        }
        Some(crate::config::Feed::Both) => {
            let atom = capsule.as_atom(&cfg)?;
            let dest = Feed::get_path(&output, None);
            atom.to_disk(&dest)?;
            let feed = capsule.gemfeed(&cfg)?;
            let dest = GemFeed::get_path(&output, None);
            feed.to_disk(&dest)?;
        }
        None => {}
    }
    capsule.render_tags(&cfg, &output)?;
    capsule.render_index(&cfg, &output)?;
    capsule.render_gemlog_index(&cfg, &output)?;
    Ok(())
}

/// The metadata extracted from all posts and pages used to construct the rest
/// of the site
struct Capsule {
    posts: Posts,
    tags: Tags,
    banner: Option<String>,
}

impl AsAtom<Feed> for Capsule {
    type Err = crate::Error;

    /// Generates an Atom feed from the metadata
    fn as_atom(&self, cfg: &Config) -> Result<Feed, Self::Err> {
        let mut entries: Vec<atom::Entry> = vec![];
        for entry in self.posts.values().rev() {
            entries.push(entry.as_atom(cfg)?);
        }
        let year = if let Some(Some(date)) = self
            .posts
            .values()
            .last()
            .map(|post| post.meta.published.as_ref())
        {
            date.year()
        } else {
            Time::now().year()
        };
        let mut url = cfg.url()?;
        if let Some(p) = &cfg.path {
            url.set_path(p);
        }
        let feed = atom::FeedBuilder::default()
            .title(cfg.title.to_string())
            .id(url.to_string())
            .author(cfg.author.to_atom())
            .rights(atom::Text::plain(format!(
                "© {} by {}",
                year, &cfg.author.name
            )))
            .base(url.to_string())
            .entries(entries)
            .build();
        Ok(feed)
    }
}

impl Capsule {
    /// Walks the "content" directory tree and extracts all of the information
    /// required to build the site. All pages and gemlog posts are also rendered
    /// in this function's main loop for efficiency.
    fn init(cfg: &Config, output: &Path) -> Result<Self, crate::Error> {
        let mut posts: Posts = BTreeMap::new();
        let mut tags: Tags = HashMap::new();
        let mut current = std::env::current_dir()?;
        current.push("content");
        if !current.exists() {
            std::fs::create_dir_all(&current)?;
        }
        let current = std::fs::canonicalize(&current)?;
        let mut index = current.clone();
        index.push("index.gmi");
        let mut gemlog_index = current.clone();
        gemlog_index.push("gemlog");
        gemlog_index.push("index.gmi");
        let banner = match crate::banner::get() {
            Some(Ok(s)) => Some(s.trim_end().to_string()),
            Some(Err(e)) => {
                eprintln!("Error reading banner file");
                return Err(e.into());
            }
            None => None,
        };
        for entry in WalkDir::new("content").into_iter().flatten() {
            let path = PathBuf::from(entry.path());
            let path = std::fs::canonicalize(&path)?;
            let last = path.strip_prefix(&current)?;
            if let Some(n) = last.to_str() {
                if n == "index.gmi" || n == "gemlog/index.gmi" {
                    continue;
                }
            }
            let mut output = output.to_path_buf();
            output.push(&last);
            if let Some(parent) = output.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            if let Some(s) = path.extension() {
                if let Some("gmi") = s.to_str() {
                    if let Some(page) = Page::from_path(&path) {
                        if let Some(ref time) = page.meta.published {
                            if path != index && path != gemlog_index {
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
                                    page.render(cfg, &output, depth, &banner)?;
                                    let post = Post {
                                        link,
                                        meta: page.meta.clone(),
                                    };
                                    posts.insert(time.timestamp()?, post);
                                } else {
                                    page.render(cfg, &output, depth, &banner)?;
                                }
                            }
                        }
                    }
                } else if entry.file_type().is_file() {
                    std::fs::copy(&path, &output)?;
                }
            } else if entry.file_type().is_file() {
                std::fs::copy(&path, &output)?;
            }
        }
        Ok(Self {
            posts,
            tags,
            banner,
        })
    }

    /// Generates a Gemini feed from the metadata
    fn gemfeed(&self, cfg: &Config) -> Result<GemFeed, crate::Error> {
        let mut page = format!("# {}\n\n", &cfg.title);
        for entry in self.posts.values().rev() {
            writeln!(page, "{}", entry.link,)?;
        }
        Ok(GemFeed(page))
    }

    /// Creates a gemtext page for each tag and an index page of all tags
    fn render_tags(&self, cfg: &Config, output: &Path) -> Result<(), crate::Error> {
        let index_path = Index::get_path(output, Some(&PathBuf::from("tags")));
        let base_url = cfg.url()?;
        let tags_url = base_url.join("tags/")?;
        let mut index_page = match &self.banner {
            Some(s) => format!("```\n{}\n```# {}\n\n### All tags\n", s, &cfg.title),
            None => format!("# {}\n\n### All tags\n", &cfg.title),
        };
        let mut dest = PathBuf::from(output);
        dest.push("tags");
        if !dest.exists() {
            fs::create_dir_all(&dest)?;
        }
        for (tag, links) in &self.tags {
            writeln!(index_page, "=> {}.gmi {}", &tag, &tag)?;
            let mut dest = dest.clone();
            dest.push(tag);
            dest.set_extension("gmi");
            let mut page = match &self.banner {
                Some(s) => format!(
                    "```\n{s}\n```# {}\n\n### Pages tagged {}\n",
                    &cfg.title, &tag
                ),
                None => format!("# {}\n\n### Pages tagged {}\n", &cfg.title, &tag),
            };
            for link in links {
                let url = if let Some(u) = tags_url.make_relative(&Url::parse(&link.url)?) {
                    Cow::from(u.to_string())
                } else {
                    Cow::from(&link.url)
                };
                writeln!(page, "=> {url} {}", link.display)?;
            }
            page.push_str("\n=> . All tags\n=> .. Home\n");
            let year = Utc::now().date().year();
            crate::footer(&mut page, year, cfg)?;
            fs::write(&dest, &page.as_bytes())?;
        }
        index_page.push_str("\n=> .. Home\n");
        let year = Utc::now().date().year();
        crate::footer(&mut index_page, year, cfg)?;
        Index(index_page).to_disk(&index_path)?;
        Ok(())
    }

    /// Renders the capsule main index
    fn render_index(&self, cfg: &Config, output: &Path) -> Result<(), crate::Error> {
        let origin: PathBuf = ["content", "index.gmi"].iter().collect();
        let page = if let Some(p) = Page::from_path(&origin) {
            p
        } else {
            let mut idx = Page::default();
            idx.content.push_str("{% posts %}");
            idx
        };
        let mut content = match &self.banner {
            Some(s) => format!("```\n{}\n```# {}\n\n", s, &cfg.title),
            None => format!("# {}\n\n", &cfg.title),
        };
        content.push_str(&page.content);
        content.push('\n');
        let mut posts = String::from("### Gemlog posts\n");
        let num = std::cmp::min(cfg.entries, self.posts.len());
        let base = cfg.url()?;
        for post in self.posts.values().rev().take(num) {
            let url = Url::parse(&post.link.url)?;
            let url = if let Some(u) = base.make_relative(&url) {
                Cow::from(u.to_string())
            } else {
                Cow::from(&post.link.url)
            };
            writeln!(posts, "=> {url} {}", post.link.display)?;
        }
        posts.push_str("=> gemlog/ All posts\n");
        let mut content = content.replace("{% posts %}", &posts);
        let year = Utc::now().date().year();
        crate::footer(&mut content, year, cfg)?;
        let path = Index::get_path(&PathBuf::from(output), None);
        Index(content).to_disk(&path)?;
        Ok(())
    }

    /// Renders the gemlog index
    fn render_gemlog_index(&self, cfg: &Config, output: &Path) -> Result<(), crate::Error> {
        let origin: PathBuf = ["content", "gemlog", "index.gmi"].iter().collect();
        let page = if let Some(p) = Page::from_path(&origin) {
            p
        } else {
            Page::default()
        };
        let mut content = match &self.banner {
            Some(s) => format!("```\n{}\n```# {}\n\n", s, &cfg.title),
            None => format!("# {}\n\n", &cfg.title),
        };
        content.push_str(&page.content);
        content.push_str("\n\n### Gemlog posts\n");
        let base = cfg.url()?;
        let base = base.join("gemlog/index.gmi")?;
        for post in self.posts.values().rev() {
            let url = Url::parse(&post.link.url)?;
            let url = if let Some(u) = base.make_relative(&url) {
                Cow::from(u.to_string())
            } else {
                Cow::from(&post.link.url)
            };
            writeln!(content, "=> {url} {}", post.link.display)?;
        }
        match &cfg.feed {
            Some(crate::config::Feed::Atom) => {
                writeln!(content, "\n=> atom.xml Atom Feed")?;
            }
            Some(crate::config::Feed::Gemini) => {
                writeln!(content, "\n=> feed.gmi Gemini Feed")?;
            }
            Some(crate::config::Feed::Both) => {
                writeln!(content, "\n=> atom.xml Atom Feed\n=> feed.gmi Gemini Feed")?;
            }
            None => {}
        }
        writeln!(content, "\n=> ../tags tags\n=> .. Home")?;
        let year = Utc::now().date().year();
        crate::footer(&mut content, year, cfg)?;
        let path = Index::get_path(&PathBuf::from(output), Some(&PathBuf::from("gemlog")));
        Index(content).to_disk(&path)?;
        Ok(())
    }
}
