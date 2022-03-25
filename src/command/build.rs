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
                std::fs::write(path, &atom);
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
    Ok(())
}

fn items(cfg: &Config, output: &PathBuf) -> Result<(BTreeMap<i64, Meta>, HashMap<PathBuf, Meta>), Box<dyn Error>> {
    let mut posts: BTreeMap<i64, Meta> = BTreeMap::new();
    let mut pages: HashMap<PathBuf, Meta> = HashMap::new();
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
            let mut output = output.clone();
            output.push(&last);
            if let Some(s) = path.extension() {
                if let Some("gmi") = s.to_str() {
                    if let Some(page) = Page::from_path(&path) {
                        if let Some(ref time) = page.meta.published {
                            if &path != &index && &path != &gemlog_index {
                                let path = path.strip_prefix(&current)?;
                                if path.starts_with("gemlog") {
                                    page.render(cfg, &output)?;
                                    posts.insert(time.timestamp()?, page.meta);
                                } else {
                                    page.render(cfg, &output)?;
                                    pages.insert(path.to_path_buf(), page.meta);
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
    Ok((posts, pages))
}

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
}
