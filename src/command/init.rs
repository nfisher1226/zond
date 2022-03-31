use {
    crate::{
        config::{Config, Feed},
        content::Page,
        traits::ToDisk,
    },
    clap::ArgMatches,
    std::{error::Error, path::PathBuf},
    url::Url,
};

/// Creates and saves `Config.ron` to disk
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
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
    cfg.path = matches.value_of("path").map(|x| x.to_string());
    if let Some(e) = matches.value_of("entries") {
        cfg.entries = e.parse()?;
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
        cfg.show_email = e.parse()?;
    }
    if !cfg_file.exists() {
        cfg.save()?;
    }
    let mut gemlog = PathBuf::from("content");
    gemlog.push("gemlog");
    if !gemlog.exists() {
        std::fs::create_dir_all(&gemlog)?;
    }
    let mut idx = PathBuf::from("content");
    idx.push("index.gmi");
    let mut idx_page = Page::default();
    idx_page.content.push_str("{% posts %}\n");
    idx_page.to_disk(&idx)?;
    idx = PathBuf::from("content");
    idx.push("gemlog");
    idx.push("index.gmi");
    idx_page = Page::default();
    idx_page.to_disk(&idx)?;
    Ok(())
}
