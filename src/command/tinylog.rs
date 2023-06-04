use {
    crate::{tinylog, Error},
    clap::ArgMatches,
};

/// Manages the tinylog
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), Error> {
    let tags = matches
        .get_many::<String>("tags")
        .map(|t| t.map(ToString::to_string).collect::<Vec<_>>());
    if let Some(post) = matches.get_one::<String>("post") {
        tinylog::update(&post.to_string(), tags)?;
    } else if let Some(tags) = tags {
        tinylog::tags(&tags)?;
    } else if matches.get_flag("edit") {
        tinylog::edit()?;
    } else {
        tinylog::create_post()?;
    }
    Ok(())
}
