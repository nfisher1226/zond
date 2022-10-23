use std::{env, process::Command};

/// Open the given uri in an appropriate program
pub fn edit(file: &str) -> Result<(), crate::Error> {
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

fn run(handler: &str, arg: &str) -> Result<(), crate::Error> {
    if let Err(e) = Command::new(handler).arg(arg).status() {
        Err(crate::Error::EditorError(format!("{e}")))
    } else {
        Ok(())
    }
}
