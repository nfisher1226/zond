use {
    crate::{
        config::{Config, Feed},
        content::{Meta, Page},
        ToDisk,
    },
    clap::ArgMatches,
    std::path::PathBuf,
    url::Url,
};

/// Creates and saves `Config.ron` to disk
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let cfg_file = PathBuf::from("Config.ron");
    let mut cfg = Config::default();
    if let Some(title) = matches.value_of("title") {
        cfg.title = title.to_string();
    }
    if let Some(author) = matches.value_of("author") {
        cfg.author.name = author.to_string();
    }
    if let Some(email) = matches.value_of("email") {
        if let Some((_user, _domain)) = email.split_once('@') {
            cfg.author.email = Some(email.to_string());
        } else {
            return Err(format!("Invalid email address: {}", email).into());
        }
    }
    if let Some(addr) = matches.value_of("url") {
        if let Ok(url) = Url::parse(addr) {
            cfg.author.url = Some(url.to_string());
        } else {
            return Err(format!("Invalid url: {}", addr).into());
        }
    }
    if let Some(domain) = matches.value_of("domain") {
        cfg.domain = domain.to_string();
    }
    cfg.path = matches
        .value_of("path")
        .map(std::string::ToString::to_string);
    if let Some(e) = matches.value_of("entries") {
        cfg.entries = match e.parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error parsing number for entry display: invalid string");
                return Err(e.into());
            }
        };
    }
    if let Some(d) = matches.value_of("display_email") {
        cfg.display_date = d.parse()?;
    }
    if let Some(f) = matches.value_of("feed") {
        match f {
            "Atom" | "atom" => cfg.feed = Some(Feed::Atom),
            "Gemini" | "gemini" => cfg.feed = Some(Feed::Gemini),
            "Both" | "both" => cfg.feed = Some(Feed::Both),
            s => return Err(format!("Invalid string: {}", s).into()),
        }
    }
    if let Some(l) = matches.value_of("license") {
        cfg.license = Some(l.into());
    }
    if let Some(e) = matches.value_of("show_email") {
        cfg.show_email = match e.parse() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error parsing input: invalid boolean");
                return Err(e.into());
            }
        };
    }
    if !cfg_file.exists() {
        cfg.save()?;
    }
    let mut gemlog = PathBuf::from("content");
    gemlog.push("gemlog");
    if !gemlog.exists() {
        if let Err(e) = std::fs::create_dir_all(&gemlog) {
            eprintln!("Error creating gemlog content directory");
            return Err(e.into());
        }
    }
    let mut idx: PathBuf = ["content", "index.gmi"].iter().collect();
    let mut idx_page = Page {
        meta: Meta::default(),
        content: String::from("{% posts %}"),
    };
    idx_page.to_disk(&idx)?;
    idx = ["content", "gemlog", "index.gmi"].iter().collect();
    idx_page = Page::default();
    idx_page.to_disk(&idx)?;
    Ok(())
}
