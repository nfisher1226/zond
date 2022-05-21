#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]
/// Adds an ascii banner to each page (if the file banner.txt exists)
pub(crate) mod banner;
/// Generates the command line options struct
mod cli;
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
    let matches = cli::zond().get_matches();
    command::run(&matches)?;
    Ok(())
}
