use {
    clap_complete::{generate_to, shells},
    clap_mangen::Man,
    std::{env, error::Error, fs, path::PathBuf, process},
};

include!("../../src/cli.rs");

fn completions() -> Result<(), Box<dyn Error>> {
    println!("Generating completions:");
    let mut cmd = build();
    let outdir: PathBuf = ["target", "dist", "share", "bash-completion", "completions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::Bash, &mut cmd, "zond", outdir)?;
    println!("    {}", path.display());
    let outdir: PathBuf = ["target", "dist", "share", "zsh", "site-functions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::Zsh, &mut cmd, "zond", outdir)?;
    println!("    {}", path.display());
    let outdir: PathBuf = ["target", "dist", "share", "fish", "completions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::Fish, &mut cmd, "zond", outdir.to_path_buf())?;
    println!("    {}", path.display());
    // Disabling this for now because I don't know where powershell looks for completions
    let outdir: PathBuf = ["target", "dist", "share", "pwsh", "completions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::PowerShell, &mut cmd, "zond", outdir.to_path_buf())?;
    println!("    {}", path.display());
    Ok(())
}

fn manpage(cmd: &str) -> Result<(), Box<dyn Error>> {
    let (fname, cmd) = match cmd {
        "zond-build" => ("zond-build.1", build_build()),
        "zond-init" => ("zond-init.1", build_init()),
        "zond-page" => ("zond-page.1", build_page()),
        "zond-post" => ("zond-post.1", build_post()),
        _ => ("zond.1", build()),
    };
    let outdir: PathBuf = ["target", "dist", "share", "man", "man1"].iter().collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let mut outfile = outdir;
    outfile.push(fname);
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(&outfile, buffer)?;
    println!("    {}", outfile.display());
    Ok(())
}

fn manpages() -> Result<(), Box<dyn Error>> {
    println!("Generating man pages:");
    ["zond", "zond-build", "zond-init", "zond-page", "zond-post"]
        .iter()
        .try_for_each(|cmd| manpage(cmd))?;
    Ok(())
}

fn usage() {
    println!("Usage: xtask dist");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        process::exit(0);
    }
    if &args[1] == "dist" {
        let outdir: PathBuf = ["target", "dist"].iter().collect();
        if outdir.exists() {
            fs::remove_dir_all(&outdir)?;
        }
        completions()?;
        manpages()?;
    } else {
        usage();
    }
    Ok(())
}
