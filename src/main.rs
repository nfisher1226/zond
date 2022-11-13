use gettextrs::*;

fn main() -> Result<(), zond::Error> {
    TextDomain::new("zond")
        .push("/usr/local/share")
        .init()
        .expect("Error initializing locale");
    bind_textdomain_codeset("zond", "UTF-8")?;
    let matches = zond::cli::zond().get_matches();
    zond::command::run(&matches)?;
    Ok(())
}
