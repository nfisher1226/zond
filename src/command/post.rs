use {
    crate::content,
    clap::ArgMatches,
    gettextrs::gettext,
};

/// Matches the `post` subcommand cli arguments and runs the appropriate code
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let title = match matches.get_one::<String>("title") {
        Some(t) => t,
        None => return Err(String::from(gettext("Missing title")).into()),
    };
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let tags = match init_matches.get_many::<String>("tags") {
                Some(t) => t.map(std::string::ToString::to_string).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            content::Page::create(
                content::Kind::Post,
                title,
                init_matches.get_one::<String>("summary").map(|x| &**x),
                tags,
            )?;
            if init_matches.get_flag("edit") {
                content::Page::edit(content::Kind::Post, title)?;
            }
        }
        Some(("publish", _publish_matches)) => {
            content::Page::publish(content::Kind::Post, title)?;
        }
        Some(("edit", _edit_matches)) => {
            content::Page::edit(content::Kind::Post, title)?;
        }
        _ => {}
    }
    Ok(())
}
