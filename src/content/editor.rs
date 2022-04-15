use std::{env, error::Error, process::Command};

/// Open the given uri in an appropriate program
pub fn edit(file: &str) -> Result<(), Box<dyn Error>> {
    if let Ok(ed) = env::var("EDITOR") {
        run(&ed, file)
    } else {
        run("vi", file)
            .or_else(|_| run("vim", file))
            .or_else(|_| run("emacs", file))
            .or_else(|_| run("nano", file))
            .or_else(|_| run("ee", file))
    }?;
    Ok(())
}

fn run(handler: &str, arg: &str) -> Result<(), Box<dyn Error>> {
    Command::new(handler).arg(arg).status()?;
    Ok(())
}
