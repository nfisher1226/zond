use {
    crate::content,
    clap::ArgMatches,
    std::{error::Error, path::PathBuf},
};

/// Matches the `page` subcommand cli arguments and runs the appropriate code
///
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let title = matches.value_of("title").unwrap_or("");
    let path = matches.value_of("path").map(PathBuf::from);
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let tags = match init_matches.values_of("tags") {
                Some(t) => t.map(std::string::ToString::to_string).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            content::Page::create(
                content::Kind::Page(path.clone()),
                title,
                init_matches.value_of("summary"),
                tags,
            )?;
            if init_matches.is_present("edit") {
                content::Page::edit(content::Kind::Page(path), title)?;
            }
        }
        Some(("publish", _publish_matches)) => {
            content::Page::publish(content::Kind::Page(path), title)?;
        }
        Some(("edit", _edit_matches)) => {
            content::Page::edit(content::Kind::Page(path), title)?;
        }
        _ => {}
    }
    Ok(())
}
