#![allow(dead_code)]

mod cli;
use {
    clap::{Arg, Command},
    package_bootstrap::Bootstrap,
    pulldown_cmark::{html, Parser},
    std::{env, error::Error, fs, path::PathBuf, process},
};

fn docs(outdir: &str) -> Result<(), std::io::Error> {
    println!("Installing documentation:");
    let docs = [
        "build.md",
        "customizing.md",
        "index.md",
        "page.md",
        "post.md",
        "tinylog.md",
    ];
    let outdir: PathBuf = [outdir, "share", "doc", "zond"].iter().collect();
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

fn compile_translation(outdir: &str, potfile: &str, lang: &str) -> Result<(), Box<dyn Error>> {
    let infile: PathBuf = ["po", potfile].iter().collect();
    let lcdir: PathBuf = [outdir, "share", "locale", lang, "LC_MESSAGES"]
        .iter()
        .collect();
    if !lcdir.exists() {
        fs::create_dir_all(&lcdir)?;
    }
    let mut outfile = lcdir.clone();
    outfile.push("zond.mo");
    let output = process::Command::new("msgfmt")
        .args([infile.to_str().unwrap(), "-o", outfile.to_str().unwrap()])
        .output()?;
    assert!(output.status.success());
    println!("    {} -> {}", infile.display(), outfile.display());
    Ok(())
}

fn translations(outdir: &str) -> Result<(), Box<dyn Error>> {
    println!("Compiling translations:");
    compile_translation(outdir, "it.po", "it")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("bootstrap")
        .about("install the software")
        .author("Nathan Fisher")
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("target-dir")
                .help("the directory where the 'gfret' binary is located")
                .short('t')
                .long("target-dir")
                .num_args(1),
            Arg::new("output")
                .help("the output directory for the installation")
                .required(true)
                .num_args(1),
        ])
        .get_matches();
    let outdir = matches.get_one::<String>("output").unwrap().to_string();
    let out = PathBuf::from(&outdir);
    let target_dir = matches
        .get_one::<String>("target-dir")
        .map(|x| x.to_string());
    Bootstrap::new("zond", cli::zond(), &out).install(target_dir, 1)?;
    Bootstrap::new("zond-init", cli::init(), &out).manpage(1)?;
    Bootstrap::new("zond-build", cli::build(), &out).manpage(1)?;
    Bootstrap::new("zond-post", cli::post(), &out).manpage(1)?;
    Bootstrap::new("zond-post-init", cli::post_init(), &out).manpage(1)?;
    Bootstrap::new("zond-page", cli::page(), &out).manpage(1)?;
    Bootstrap::new("zond-page-init", cli::page_init(), &out).manpage(1)?;
    Bootstrap::new("zond-tinylog", cli::tinylog(), &out).manpage(1)?;
    docs(&outdir)?;
    translations(&outdir)?;
    Ok(())
}
