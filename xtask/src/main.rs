use {
    clap_complete::{generate_to, shells},
    clap_mangen::Man,
    pulldown_cmark::{html, Parser},
    std::{env, error::Error, fs, path::PathBuf, process},
};

include!("../../src/cli.rs");

fn docs() -> Result<(), std::io::Error> {
    println!("Installing documentation:");
    let docs = ["build.md", "customizing.md", "index.md", "page.md", "post.md"];
    let outdir: PathBuf = ["target", "dist", "share", "doc", "zond"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    for doc in docs {
        let mut outfile = outdir.clone();
        outfile.push(doc);
        outfile.set_extension("html");
        let infile: PathBuf = ["doc", doc].iter().collect();
        let mdstr = fs::read_to_string(&infile)?;
        let mdstr = mdstr.replace(".md", ".html");
        let parser = Parser::new(&mdstr);
        let fd = fs::File::create(&outfile)?;
        html::write_html(fd, parser)?;
        println!("    {} -> {}", infile.display(), outfile.display());
    }
    Ok(())
}

fn completions() -> Result<(), Box<dyn Error>> {
    println!("Generating completions:");
    let mut cmd = zond();
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
        "zond-build" => ("zond-build.1", build()),
        "zond-init" => ("zond-init.1", init()),
        "zond-page" => ("zond-page.1", page()),
        "zond-post" => ("zond-post.1", post()),
        "zond-page-init" => ("zond-page-init.1", page_init()),
        "zond-post-init" => ("zond-post-init.1", post_init()),
        _ => ("zond.1", zond()),
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
    ["zond", "zond-build", "zond-init", "zond-page", "zond-post", "zond-page-init", "zond-post-init"]
        .iter()
        .try_for_each(|cmd| manpage(cmd))?;
    Ok(())
}

fn compile_translation(potfile: &str, lang: &str) -> Result<(), Box<dyn Error>> {
    let infile: PathBuf = ["po", potfile].iter().collect();
    let lcdir: PathBuf = ["target", "dist", "share", "locale", lang, "LC_MESSAGES"]
        .iter()
        .collect();
    if !lcdir.exists() {
        fs::create_dir_all(&lcdir)?;
    }
    let mut outfile = lcdir.clone();
    outfile.push("zond.mo");
    let output = process::Command::new("msgfmt")
        .args([
            infile.to_str().unwrap(),
            "-o",
            outfile.to_str().unwrap(),
        ])
        .output()?;
    assert!(output.status.success());
    println!("    {} -> {}", infile.display(), outfile.display());
    Ok(())
}

fn translations() -> Result<(), Box<dyn Error>> {
    println!("Compiling translations:");
    compile_translation("it.po", "it")?;
    Ok(())
}

fn copy_bin() -> Result<(), Box<dyn Error>> {
    println!("Copying binary:");
    let bindir: PathBuf = ["target", "dist", "bin"].iter().collect();
    if !bindir.exists() {
        fs::create_dir_all(&bindir)?;
    }
    let mut outfile = bindir;
    outfile.push("zond");
    let infile: PathBuf = ["target", "release", "zond"].iter().collect();
    if !infile.exists() {
        eprintln!("Error: you must run \"cargo build --release\" first");
    }
    fs::copy(&infile, &outfile)?;
    println!("    {} -> {}", infile.display(), outfile.display());
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
        copy_bin()?;
        completions()?;
        manpages()?;
        docs()?;
        translations()?;
    } else {
        usage();
    }
    Ok(())
}
