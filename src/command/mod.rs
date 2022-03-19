pub mod build;
pub mod init;
pub mod page;
pub mod post;

use {
    clap::ArgMatches,
    std::error::Error,
};

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some(("init", init_matches)) => init::run(init_matches)?,
        Some(("page", page_matches)) => page::run(page_matches)?,
        Some(("post", post_matches)) => post::run(post_matches)?,
        Some(("build",build_matches)) => build::run(build_matches)?,
        _ => {},
    }
    Ok(())
}
