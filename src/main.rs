fn main() -> Result<(), zond::Error> {
    let matches = zond::cli::zond().get_matches();
    zond::command::run(&matches)?;
    Ok(())
}
