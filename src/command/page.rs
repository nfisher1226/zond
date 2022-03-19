use {
    clap::ArgMatches,
    std::{ error::Error, path::PathBuf },
    crate::content,
};

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let title = match matches.value_of("title") {
        Some(t) => t,
        None => return Err(String::from("Missing title").into()),
    };
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let path = match init_matches.value_of("path") {
                Some(p) => Some(PathBuf::from(p)),
                None => None,
            };
            let tags = match init_matches.values_of("tags") {
                Some(t) => t.map(|x| x.to_string()).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            content::Page::create(
                content::Kind::Page(path),
                &title,
                init_matches.value_of("summary"),
                tags,
            )?;
        },
        Some(("publish", publish_matches)) => {
            let path = match publish_matches.value_of("path") {
                Some(p) => Some(PathBuf::from(p)),
                None => None,
            };
            content::Page::publish(content::Kind::Page(path), &title)?;
        },
        _ => {},
    }
    Ok(())
}
