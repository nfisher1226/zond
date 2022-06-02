use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = zond::cli::zond().get_matches();
    zond::command::run(&matches)?;
    Ok(())
}
