use {
    clap_complete::{generate_to, shells},
    std::{env, io::Error},
};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let mut cmd = build();
    let _path = generate_to(shells::Bash, &mut cmd, "vostok", outdir.clone())?;
    let _path = generate_to(shells::Zsh, &mut cmd, "vostok", outdir.clone())?;
    let _path = generate_to(shells::Fish, &mut cmd, "vostok", outdir.clone())?;
    let _path = generate_to(shells::PowerShell, &mut cmd, "vostok", outdir.clone())?;
    println!(
        "cargo:warning=Shell completions have been saved in: {:?}",
        outdir
    );
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
