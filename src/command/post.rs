use {crate::content, clap::ArgMatches, std::error::Error};

/// Matches the `post` subcommand cli arguments and runs the appropriate code
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let title = match matches.value_of("title") {
        Some(t) => t,
        None => return Err(String::from("Missing title").into()),
    };
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let tags = match init_matches.values_of("tags") {
                Some(t) => t.map(std::string::ToString::to_string).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            content::Page::create(
                content::Kind::Post,
                title,
                init_matches.value_of("summary"),
                tags,
            )?;
            if init_matches.is_present("edit") {
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
