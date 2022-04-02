#[cfg(feature = "completions")]
use {
    clap_complete::{generate_to, shells},
    std::env,
};

use std::io::Error;

#[cfg(feature = "completions")]
include!("src/cli.rs");

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
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
