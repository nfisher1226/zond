#![doc = include_str!("../README.md")]
/// Generates the command line options struct
mod cli;
/// Adds an ascii banner to each page (if the file banner.txt exists)
pub(crate) mod banner;
/// Parses out the subcommands from the cli
pub(crate) mod command;
/// Holds the capsule level configuration
pub(crate) mod config;
/// Working with pages and gemlog posts
pub(crate) mod content;
/// Common traits
pub(crate) mod traits;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::build().get_matches();
    command::run(&matches)?;
    Ok(())
}
