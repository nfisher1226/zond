//! Running concurrently to NASA's project Mercury, the Soviet *Vostok*, or if you
//! prefer the original Russian, *Восток* project was a human spaceflight program
//! with the goal of putting the first human into low earth orbit and returning them
//! safely. Yuri Gagarin in fact became the first human to do so on April 12th, 1961.
//!
//! In this context, Vostok is a static gemini capsule generator, think static site
//! generator like Hugo or Jekyll but for [Project Gemini](https://gemini.circumlunar.space).
//!
//! ## Status
//! This software is currently under construction and does not yet perform it's
//! intended task. Please check back later.
pub mod command;
pub mod config;
pub mod content;

use {
    clap::{Arg, Command},
    std::error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("vostok")
        .about("A static Gemini capsule generator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new capsule")
                .arg(
                    Arg::new("wizard")
                        .short('w')
                        .long("wizard")
                        .help("Run the interactive wizard")
                        .takes_value(false)
                )
        )
        .subcommand(
            Command::new("build")
                .about("Build the capsule")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("The location to output the generated capsule")
                        .takes_value(true)
                        .multiple_values(false)
                )
        )
        .subcommand(
            Command::new("post")
                .about("Create a new gemlog post")
                .arg(
                    Arg::new("title")
                        .help("The title of the page")
                        .takes_value(true)
                        .multiple_values(false)
                        .required(true)
                )
                .subcommand(
                    Command::new("init")
                        .about("Initializes a new post")
                        .arg(
                            Arg::new("summary")
                                .help("A short summary of the post (optional)")
                                .short('s')
                                .long("summary")
                                .takes_value(true)
                                .multiple_values(false)
                                .required(false)
                        )
                        .arg(
                            Arg::new("tags")
                                .help("Tags for this post (optional)")
                                .short('t')
                                .long("tags")
                                .takes_value(true)
                                .multiple_values(true)
                                .required(false)
                        )
                )
                .subcommand(
                    Command::new("publish")
                        .about("Marks the post as published")
                )
        )
        .subcommand(
            Command::new("page")
                .about("Create a new page")
                .arg(
                    Arg::new("title")
                        .help("The title of the page")
                        .takes_value(true)
                        .multiple_values(false)
                        .required(true)
                )
                .subcommand(
                    Command::new("init")
                        .about("Initializes a new page")
                        .arg(
                            Arg::new("path")
                                .help("The path from the capsule root (optional)")
                                .short('p')
                                .long("path")
                                .takes_value(true)
                                .multiple_values(false)
                                .required(false)
                        )
                        .arg(
                            Arg::new("summary")
                                .help("A short summary of the page (optional)")
                                .short('s')
                                .long("summary")
                                .takes_value(true)
                                .multiple_values(false)
                                .required(false)
                        )
                        .arg(
                            Arg::new("tags")
                                .help("Tags for this page (optional)")
                                .short('t')
                                .long("tags")
                                .takes_value(true)
                                .multiple_values(true)
                                .required(false)
                        )
                )
                .subcommand(
                    Command::new("publish")
                        .about("Marks the page as published")
                        .arg(
                            Arg::new("path")
                                .help("The path from the capsule root (optional)")
                                .short('p')
                                .long("path")
                                .takes_value(true)
                                .multiple_values(false)
                                .required(false)
                        )
                )
        )
        .get_matches();

    command::run(&matches)?;
    Ok(())
}
