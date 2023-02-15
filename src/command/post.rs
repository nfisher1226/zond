use {
    crate::content::{Kind, Page},
    clap::ArgMatches,
    std::string::ToString,
};

/// Matches the `post` subcommand cli arguments and runs the appropriate code
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    let Some(title) = matches.get_one::<String>("title") else {
        return Err(String::from("Missing title").into());
    };
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let tags = match init_matches.get_many::<String>("tags") {
                Some(t) => t.map(ToString::to_string).collect::<Vec<_>>(),
                None => Vec::new(),
            };
            Page::create(
                Kind::Post,
                title,
                init_matches.get_one::<String>("summary").map(|x| &**x),
                tags,
            )?;
            if init_matches.get_flag("edit") {
                Page::edit(Kind::Post, title)?;
            }
        }
        Some(("publish", _publish_matches)) => {
            Page::publish(Kind::Post, title)?;
        }
        Some(("edit", _edit_matches)) => {
            Page::edit(Kind::Post, title)?;
        }
        _ => {}
    }
    Ok(())
}
