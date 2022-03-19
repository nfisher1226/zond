use {
    atom_syndication as atom,
    atom_syndication::Feed,
    clap::ArgMatches,
    crate::{
        config::Config,
        content::{
            Kind,
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
};

struct Capsule {
    path: PathBuf,
    posts: BTreeMap<i64, Page>,
    pages: HashMap<String, Page>,
}

impl Capsule {
    fn init(path: &str) -> Self {
        let posts = posts();
        let pages = pages();
        let path = PathBuf::from(path);
        Self {
            path,
            posts,
            pages,
        }
    }

    fn gen_rss(&self, cfg: &Config) -> Result<(), Box<dyn Error>> {
        let mut entries: Vec<atom::Entry> = vec![];
        for entry in posts().values().rev() {
            entries.push(entry.rss_entry(Kind::Post, cfg)?);
        }
        let year = posts()
            .values()
            .last()
            .unwrap()
            .meta
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
        Ok(())
    }

    fn build(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}


fn posts() -> BTreeMap<i64, Page> {
    let posts = BTreeMap::new();
    posts
}

fn pages() -> HashMap<String, Page> {
    let pages = HashMap::new();
    pages
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if let Some(config) = Config::load() {
        let path = matches.value_of("output").unwrap_or("public");
        let capsule = Capsule::init(path);
        capsule.build()?;
        Ok(())
    } else {
        Err("Error loading config".to_string().into())
    }
}
