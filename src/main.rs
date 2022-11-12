use gettextrs::*;

fn main() -> Result<(), zond::Error> {
    textdomain("zond")?;
    bind_textdomain_codeset("zond", "UTF-8")?;
    let matches = zond::cli::zond().get_matches();
    zond::command::run(&matches)?;
    Ok(())
}
