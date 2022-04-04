#[cfg(any(feature = "completions", feature = "manpage"))]
use std::env;
#[cfg(feature = "completions")]
use {
    clap_complete::{generate_to, shells},
};

#[cfg(feature = "manpage")]
use {
    clap_mangen::Man,
    std::path::PathBuf,
};

use std::io::Error;

#[cfg(any(feature = "completions", feature = "manpage"))]
include!("src/cli.rs");

#[cfg(feature = "manpage")]
fn build_man(cmd: &str) -> Result<(), Error> {
    let (fname, cmd) = match cmd {
        "zond-build" => ("zond-build.1", build_build()),
        "zond-init" => ("zond-init.1", build_init()),
        "zond-page" => ("zond-page.1", build_page()),
        "zond-post" => ("zond-post.1", build_post()),
        _ => ("zond.1", build()),
    };
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let file: PathBuf = [outdir.to_str().unwrap(), fname].iter().collect();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(file, buffer)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    #[cfg(feature = "completions")]
    {
        let outdir = match env::var_os("OUT_DIR") {
            None => return Ok(()),
            Some(outdir) => outdir,
        };
        let mut cmd = build();
        let _path = generate_to(shells::Bash, &mut cmd, "zond", outdir.clone())?;
        let _path = generate_to(shells::Zsh, &mut cmd, "zond", outdir.clone())?;
        let _path = generate_to(shells::Fish, &mut cmd, "zond", outdir.clone())?;
        let _path = generate_to(shells::PowerShell, &mut cmd, "zond", outdir.clone())?;
        println!(
            "cargo:warning=Shell completions have been saved in: {:?}",
            outdir
        );
    }
    #[cfg(feature = "manpage")]
    ["zond", "zond-build", "zond-init", "zond-page", "zond-post"]
        .iter()
        .try_for_each(|x| build_man(x))?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
