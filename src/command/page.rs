use {crate::content, clap::ArgMatches, std::path::PathBuf};

/// Matches the `page` subcommand cli arguments and runs the appropriate code
///
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let title = matches
        .get_one::<String>("title")
        .map_or("", std::string::String::as_str);
    let path = matches.get_one::<String>("path").map(PathBuf::from);
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let tags = match init_matches.get_many::<String>("tags") {
                Some(t) => t.map(std::string::ToString::to_string).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            content::Page::create(
                content::Kind::Page(path.clone()),
                title,
                init_matches.get_one::<String>("summary").map(|x| &**x),
                tags,
            )?;
            if init_matches.get_flag("edit") {
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
