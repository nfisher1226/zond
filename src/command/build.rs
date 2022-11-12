use {
    crate::{
        content::{index::Index, Page, Time},
        link::Link,
        post::Post,
        GetPath, ToDisk, CONFIG,
    },
    atom_syndication::{self as atom, Feed},
    chrono::{Datelike, Utc},
    clap::ArgMatches,
    gettextrs::gettext,
    std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
        fmt::Write,
        fs::{self, File},
        io::{BufWriter, Write as IoWrite},
        path::{Path, PathBuf},
    },
    url::Url,
    walkdir::WalkDir,
};

/// A `BTreeMap` of gemlog posts
type Posts = BTreeMap<i64, Post>;
/// A `HashMap` of tag names and their associated links
type Tags = HashMap<String, Vec<Link>>;

/// Performs the build
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let mut output = PathBuf::from(
        matches
            .get_one::<String>("output")
            .map_or("public", std::string::String::as_str),
    );
    if let Some(ref path) = CONFIG.path {
        output.push(path);
    }
    if !output.exists() {
        std::fs::create_dir_all(&output)?;
    }
    let output = std::fs::canonicalize(&output)?;
    if output.exists() {
        std::fs::remove_dir_all(&output)?;
    }
    let capsule = Capsule::init(&output)?;
    match CONFIG.feed {
        Some(crate::config::Feed::Atom) => {
            let atom = Feed::try_from(&capsule)?;
            let dest = Feed::get_path(&output, None);
            atom.to_disk(&dest)?;
        }
        Some(crate::config::Feed::Gemini) => {
            capsule.write_gemfeed(&output)?;
        }
        Some(crate::config::Feed::Both) => {
            let atom = Feed::try_from(&capsule)?;
            let dest = Feed::get_path(&output, None);
            atom.to_disk(&dest)?;
            capsule.write_gemfeed(&output)?;
        }
        None => {}
    }
    capsule.write_tags(&output)?;
    capsule.write_index(&output)?;
    capsule.write_gemlog_index(&output)?;
    Ok(())
}

/// The metadata extracted from all posts and pages used to construct the rest
/// of the site
struct Capsule {
    posts: Posts,
    tags: Tags,
    banner: Option<String>,
}

impl TryFrom<&Capsule> for Feed {
    type Error = crate::Error;

    /// Generates an Atom feed from the metadata
    fn try_from(capsule: &Capsule) -> Result<Feed, Self::Error> {
        let mut entries: Vec<atom::Entry> = vec![];
        for entry in capsule.posts.values().rev() {
            entries.push(entry.try_into()?);
        }
        let year = if let Some(Some(date)) = capsule
            .posts
            .values()
            .last()
            .map(|post| post.meta.published.as_ref())
        {
            date.year()
        } else {
            Time::now().year()
        };
        let mut url = CONFIG.url()?;
        if let Some(p) = &CONFIG.path {
            url.set_path(p);
        }
        let feed = atom::FeedBuilder::default()
            .title(CONFIG.title.to_string())
            .id(url.to_string())
            .author(CONFIG.author.to_atom())
            .rights(atom::Text::plain(format!(
                "© {year} {} {}",
                gettext("by"),
                &CONFIG.author.name
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
    fn init(output: &Path) -> Result<Self, crate::Error> {
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
                eprintln!("{}: {e}", gettext("Error reading banner file"));
                return Err(e.into());
            }
            None => None,
        };
        for entry in WalkDir::new("content").into_iter().flatten() {
            let path = PathBuf::from(entry.path());
            let path = std::fs::canonicalize(path)?;
            let last = path.strip_prefix(&current)?;
            if let Some(n) = last.to_str() {
                if n == "index.gmi" || n == "gemlog/index.gmi" {
                    continue;
                }
            }
            let mut output = output.to_path_buf();
            output.push(last);
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
                                let link = Link::get(&path, &page.meta)?;
                                for tag in &page.meta.tags {
                                    if let Some(t) = tags.get_mut(tag) {
                                        t.push(link.clone());
                                    } else {
                                        tags.insert(tag.to_string(), vec![link.clone()]);
                                    }
                                }
                                if last.starts_with("gemlog") {
                                    page.write(&output, depth, &banner)?;
                                    let post = Post {
                                        link,
                                        meta: page.meta.clone(),
                                    };
                                    posts.insert(time.timestamp()?, post);
                                } else {
                                    page.write(&output, depth, &banner)?;
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

    /// Creates a gemtext page for each tag and an index page of all tags
    fn write_tags(&self, output: &Path) -> Result<(), crate::Error> {
        let tags = gettext("tags");
        let index_path = Index::get_path(output, Some(&PathBuf::from(&tags)));
        let mut dest = PathBuf::from(output);
        dest.push(&tags);
        if !dest.exists() {
            fs::create_dir_all(&dest)?;
        }
        let fd = File::create(index_path)?;
        let mut writer = BufWriter::new(fd);
        let base_url = CONFIG.url()?;
        let tags_url = base_url.join(&format!("{tags}/"))?;
        match &self.banner {
            Some(s) => writeln!(
                &mut writer,
                "```\n{s}\n```# {}\n\n### {}",
                &CONFIG.title,
                gettext("All tags")
            )?,
            None => writeln!(
                &mut writer,
                "# {}\n\n### {}\n",
                &CONFIG.title,
                gettext("All tags")
            )?,
        }
        for (tag, links) in &self.tags {
            writeln!(&mut writer, "=> {}.gmi {}", &tag, &tag)?;
            let mut dest = dest.clone();
            dest.push(tag);
            dest.set_extension("gmi");
            let fd = File::create(dest)?;
            let mut tagwriter = BufWriter::new(fd);
            match &self.banner {
                Some(s) => writeln!(
                    &mut tagwriter,
                    "```\n{s}\n```# {}\n\n### {} {}",
                    &CONFIG.title,
                    gettext("Pages tagged"),
                    &tag
                )?,
                None => writeln!(
                    &mut tagwriter,
                    "# {}\n\n### {} {}",
                    &CONFIG.title,
                    gettext("Pages tagged"),
                    &tag
                )?,
            }
            for link in links {
                let url = if let Some(u) = tags_url.make_relative(&Url::parse(&link.url)?) {
                    Cow::from(u.to_string())
                } else {
                    Cow::from(&link.url)
                };
                writeln!(&mut tagwriter, "=> {url} {}", link.display)?;
            }
            writeln!(
                &mut tagwriter,
                "=> . {}\n=> .. {}",
                gettext("All tags"),
                gettext("Home"),
            )?;
            let year = Utc::now().date().year();
            crate::write_footer(&mut tagwriter, year)?;
        }
        writeln!(&mut writer, "\n=> .. Home")?;
        let year = Utc::now().date().year();
        crate::write_footer(&mut writer, year)?;
        Ok(())
    }

    /// Renders the capsule main index and writes it to disk
    fn write_index(&self, output: &Path) -> Result<(), crate::Error> {
        let origin: PathBuf = ["content", "index.gmi"].iter().collect();
        let outfile = Index::get_path(&PathBuf::from(output), None);
        let fd = File::create(outfile)?;
        let mut writer = BufWriter::new(fd);
        let page = if let Some(p) = Page::from_path(&origin) {
            p
        } else {
            let mut idx = Page::default();
            idx.content.push_str("{% posts %}");
            idx
        };
        match &self.banner {
            Some(s) => writeln!(&mut writer, "```\n{s}\n```# {}\n", &CONFIG.title)?,
            None => writeln!(&mut writer, "# {}\n", &CONFIG.title)?,
        }
        let mut posts = format!("### {}\n", gettext("Gemlog posts"));
        let num = std::cmp::min(CONFIG.entries, self.posts.len());
        let base = CONFIG.url()?;
        for post in self.posts.values().rev().take(num) {
            let url = Url::parse(&post.link.url)?;
            let url = if let Some(u) = base.make_relative(&url) {
                Cow::from(u.to_string())
            } else {
                Cow::from(&post.link.url)
            };
            writeln!(posts, "=> {url} {}", post.link.display)?;
        }
        writeln!(posts, "=> gemlog/ {}\n", gettext("All posts"))?;
        let content = page.content.replace("{% posts %}", &posts);
        writeln!(&mut writer, "{content}")?;
        let year = Utc::now().date().year();
        crate::write_footer(&mut writer, year)?;
        Ok(())
    }

    /// Renders the gemlog index and writes it to disk
    fn write_gemlog_index(&self, output: &Path) -> Result<(), crate::Error> {
        let origin: PathBuf = ["content", "gemlog", "index.gmi"].iter().collect();
        let outfile = Index::get_path(&PathBuf::from(output), Some(&PathBuf::from("gemlog")));
        let fd = File::create(outfile)?;
        let mut writer = BufWriter::new(fd);
        let page = if let Some(p) = Page::from_path(&origin) {
            p
        } else {
            Page::default()
        };
        match &self.banner {
            Some(s) => write!(&mut writer, "```\n{s}\n```# {}\n\n", &CONFIG.title)?,
            None => write!(&mut writer, "# {}\n\n", &CONFIG.title)?,
        }
        write!(
            &mut writer,
            "{}\n\n### {}\n",
            &page.content,
            gettext("Gemlog posts"),
        )?;
        let base = CONFIG.url()?;
        let base = base.join("gemlog/index.gmi")?;
        for post in self.posts.values().rev() {
            let url = Url::parse(&post.link.url)?;
            let url = if let Some(u) = base.make_relative(&url) {
                Cow::from(u.to_string())
            } else {
                Cow::from(&post.link.url)
            };
            writeln!(&mut writer, "=> {url} {}", post.link.display)?;
        }
        match &CONFIG.feed {
            Some(crate::config::Feed::Atom) => {
                writeln!(&mut writer, "\n=> atom.xml {}", gettext("Atom Feed"))?;
            }
            Some(crate::config::Feed::Gemini) => {
                writeln!(&mut writer, "\n=> feed.gmi {}", gettext("Gemini Feed"))?;
            }
            Some(crate::config::Feed::Both) => {
                writeln!(
                    &mut writer,
                    "\n=> atom.xml {}\n=> feed.gmi {}",
                    gettext("Atom Feed"),
                    gettext("Gemini Feed")
                )?;
            }
            None => {}
        }
        writeln!(
            &mut writer,
            "\n=> ../{} {}\n=> .. {}",
            gettext("tags"),
            gettext("tags"),
            gettext("Home"),
        )?;
        let year = Utc::now().date().year();
        crate::write_footer(&mut writer, year)?;
        Ok(())
    }

    fn write_gemfeed(&self, output: &Path) -> Result<(), crate::Error> {
        let mut outfile = output.to_path_buf();
        outfile.push("gemlog");
        outfile.push("feed.gmi");
        let fd = File::create(outfile)?;
        let mut writer = BufWriter::new(fd);
        writeln!(&mut writer, "# {}\n", &CONFIG.title)?;
        for entry in self.posts.values().rev() {
            writeln!(&mut writer, "{}", entry.link,)?;
        }
        Ok(())
    }
}
