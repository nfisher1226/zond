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
        io::Write,
        path::{
            Path,
            PathBuf,
        },
        process::Stdio,
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

trait ToDisk {
    type Err;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err>;
}

impl ToDisk for Feed {
    type Err = Box<dyn Error>;

    fn to_disk(&self, path: &Path) -> Result<(), Self::Err> {
        if let Some(p) = path.parent() {
            if !p.exists() {
                std::fs::create_dir_all(&p)?;
            }
        }
        match std::process::Command::new("xmllint")
            .arg("-")
            .arg("--pretty")
            .arg("1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Ok(mut child) => {
                child.stdin.as_mut().unwrap().write_all(self.to_string().as_bytes())?;
                let output = child.wait_with_output()?;
                let atom = String::from_utf8_lossy(&output.stdout);
                std::fs::write(path, &String::from(atom))?;
            },
            Err(_) => {
                let atom = self.to_string();
                let atom = atom.replace(">", ">\n");
                std::fs::write(path, &atom)?;
            },
        }
        Ok(())
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
    let output = std::fs::canonicalize(&output)?;
    if output.exists() {
        std::fs::remove_dir_all(&output)?;
    }
    let capsule = Capsule::init(&cfg, output.clone())?;
    match &cfg.feed {
        Some(crate::config::Feed::Atom) => {
            let atom = capsule.atom(&cfg)?;
            let mut dest = output.clone();
            dest.push("gemlog");
            dest.push("atom.xml");
            atom.to_disk(&dest)?;
        },
        Some(crate::config::Feed::Gemini) => {
        },
        Some(crate::config::Feed::Both) => {
            let atom = capsule.atom(&cfg)?;
            let mut dest = output.clone();
            dest.push("gemlog");
            let gem = dest.clone();
            dest.push("atom.xml");
            atom.to_disk(&dest)?;
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
    path: PathBuf,
    posts: BTreeMap<i64, Meta>,
    pages: HashMap<PathBuf, Meta>,
    tags: HashMap<String, Vec<Link>>,
}

impl Capsule {
    fn init(cfg: &Config, path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let (posts, pages, tags) = items(cfg, &path)?;
        Ok(Self {
            path,
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

    fn gemfeed(&self, cfg: &Config) -> Result<String, Box<dyn Error>> {
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
        Ok(page)
    }

    fn render_tags(&self, cfg: &Config, output: &Path) -> Result<(), Box<dyn Error>> {
        let mut index = PathBuf::from(output);
        index.push(PathBuf::from("tags"));
        index.push(PathBuf::from("index.gmi"));
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
        std::fs::write(&index, &index_page.as_bytes())?;
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
            let mut index = PathBuf::from(output);
            index.push(PathBuf::from("index.gmi"));
            std::fs::write(&index, &content.as_bytes())?;
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
            let mut index = PathBuf::from(output);
            index.push(PathBuf::from("gemlog"));
            index.push(PathBuf::from("index.gmi"));
            std::fs::write(&index, &content.as_bytes())?;
        }
        Ok(())
    }
}
