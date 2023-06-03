use {
    crate::{
        config::{Config, Feed},
        content::{Meta, Page},
        ToDisk,
    },
    clap::ArgMatches,
    gettextrs::gettext,
    std::{fs, path::PathBuf, string::ToString},
    url::Url,
};

/// Creates and saves `Config.ron` to disk
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let cfg_file = PathBuf::from("Config.ron");
    let mut cfg = Config::default();
    if let Some(title) = matches.get_one::<String>("title") {
        cfg.title = title.to_string();
    }
    if let Some(author) = matches.get_one::<String>("author") {
        cfg.author.name = author.to_string();
    }
    if let Some(email) = matches.get_one::<String>("email") {
        if let Some((_user, _domain)) = email.split_once('@') {
            cfg.author.email = Some(email.to_string());
        } else {
            return Err(format!("{}: {email}", gettext("Invalid email address")).into());
        }
    }
    if let Some(addr) = matches.get_one::<String>("url") {
        if let Ok(url) = Url::parse(addr) {
            cfg.author.url = Some(url.to_string());
        } else {
            return Err(format!("{}: {addr}", gettext("Invalid url")).into());
        }
    }
    if let Some(domain) = matches.get_one::<String>("domain") {
        cfg.domain = domain.to_string();
    }
    cfg.path = matches.get_one::<String>("path").map(ToString::to_string);
    if let Some(e) = matches.get_one::<usize>("entries") {
        cfg.entries = *e;
    }
    if let Some(d) = matches.get_one::<String>("display_email") {
        cfg.display_date = d.parse()?;
    }
    if let Some(f) = matches.get_one::<String>("feed") {
        match f.as_str() {
            "Atom" | "atom" => cfg.feed = Some(Feed::Atom),
            "Gemini" | "gemini" => cfg.feed = Some(Feed::Gemini),
            "Both" | "both" => cfg.feed = Some(Feed::Both),
            s => return Err(format!("{}: {s}", gettext("Invalid string")).into()),
        }
    }
    if let Some(l) = matches.get_one::<String>("license") {
        cfg.license = Some(l.as_str().into());
    }
    if let Some(e) = matches.get_one::<String>("show_email") {
        cfg.show_email = match e.parse() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: {e}", gettext("Error parsing input"));
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
        if let Err(e) = fs::create_dir_all(&gemlog) {
            eprintln!(
                "{}: {e}",
                gettext("Error creating gemlog content directory")
            );
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
