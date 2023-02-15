use {
    crate::content::{Kind, Page},
    clap::ArgMatches,
    std::{path::PathBuf, string::ToString},
};

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
                Some(t) => t.map(ToString::to_string).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            Page::create(
                Kind::Page(path.clone()),
                title,
                init_matches.get_one::<String>("summary").map(|x| &**x),
                tags,
            )?;
            if init_matches.get_flag("edit") {
                Page::edit(Kind::Page(path), title)?;
            }
        }
        Some(("publish", _publish_matches)) => {
            Page::publish(Kind::Page(path), title)?;
        }
        Some(("edit", _edit_matches)) => {
            Page::edit(Kind::Page(path), title)?;
        }
        _ => {}
    }
    Ok(())
}
