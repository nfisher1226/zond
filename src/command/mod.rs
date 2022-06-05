/// Builds the capsule from the provided "content" directory
pub mod build;
/// Initializes a new capsule
pub mod init;
/// Standalone page operations
pub mod page;
/// Gemlog post operations
pub mod post;

use clap::ArgMatches;

/// Parses the cli and runs the appropriate subcommand
/// # Errors
/// Errors are bubbled up from the called functions
pub fn run(matches: &ArgMatches) -> Result<(), crate::Error> {
    match matches.subcommand() {
        Some(("init", init_matches)) => init::run(init_matches)?,
        Some(("page", page_matches)) => page::run(page_matches)?,
        Some(("post", post_matches)) => post::run(post_matches)?,
        Some(("build", build_matches)) => build::run(build_matches)?,
        _ => {}
    }
    Ok(())
}
