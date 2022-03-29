mod cli;
pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod content;
pub(crate) mod traits;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::build().get_matches();
    command::run(&matches)?;
    Ok(())
}
