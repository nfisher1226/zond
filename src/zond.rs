use gettextrs::*;

fn main() -> Result<(), zond::Error> {
    if let Err(e) = TextDomain::new("zond").push("/usr/local/share").init() {
        match e {
            TextDomainError::TextDomainCallFailed(_)
            | TextDomainError::BindTextDomainCallFailed(_)
            | TextDomainError::BindTextDomainCodesetCallFailed(_) => return Err(e.into()),
            _ => {}
        }
    }
    bind_textdomain_codeset("zond", "UTF-8")?;
    let matches = zond::cli::zond().get_matches();
    zond::command::run(&matches)?;
    Ok(())
}
