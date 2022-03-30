use {
    crate::content,
    clap::ArgMatches,
    std::{error::Error, path::PathBuf},
};

/// Matches the `page` subcommand cli arguments and runs the appropriate code
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let title = match matches.value_of("title") {
        Some(t) => t,
        None => "",
    };
    let path = matches.value_of("path").map(|p| PathBuf::from(p));
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let tags = match init_matches.values_of("tags") {
                Some(t) => t.map(|tag| tag.to_string()).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            content::Page::create(
                content::Kind::Page(path),
                &title,
                init_matches.value_of("summary"),
                tags,
            )?;
        }
        Some(("publish", _publish_matches)) => {
            content::Page::publish(content::Kind::Page(path), &title)?;
        }
        Some(("edit", _edit_matches)) => {
            content::Page::edit(content::Kind::Page(path), &title)?;
        }
        _ => {}
    }
    Ok(())
}
