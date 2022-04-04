use clap::{Arg, ArgGroup, Command};

pub fn build_init() -> Command<'static> {
    Command::new("init")
        .about("Initialize a new capsule")
        .arg(
             Arg::new("title")
                 .short('t')
                 .long("title")
                 .help("The title of this caspule")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("author")
                 .short('a')
                 .long("author")
                 .help("The principle author of this capsule")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("email")
                 .short('m')
                 .long("email")
                 .help("The email address of the principle author")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("url")
                 .short('u')
                 .long("url")
                 .help("The principle author's homepage")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("domain")
                 .short('d')
                 .long("domain")
                 .help("The domain serving this capsule")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("path")
                 .short('p')
                 .long("path")
                 .help("The path from the server root to this capsule")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("entries")
                 .short('e')
                 .long("entries")
                 .help("Number of gemlog entries to display links for on the homepage")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("feed")
                 .short('f')
                 .long("feed")
                 .help("The type of feed to generate. Atom, Gemini, of Both")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("license")
                 .short('l')
                 .long("license")
                 .help("Commons license to use. One of CcBy, CcBySa, CcByNc, CcByNcSa, CcByNd, CcByNcNd. For information on Creative Commons licenses, see https://creativecommons.org/about/cclicenses/")
                 .takes_value(true)
                 .multiple_values(false)
        )
        .arg(
             Arg::new("show_email")
                 .short('s')
                 .long("show_email")
                 .help("Add a link to the author's email on each page")
                 .takes_value(false)
        )
}

pub fn build_build() -> Command<'static> {
    Command::new("build").about("Build the capsule").arg(
        Arg::new("output")
            .short('o')
            .long("output")
            .help("The location to output the generated capsule")
            .takes_value(true)
            .multiple_values(false),
    )
}

pub fn build_post() -> Command<'static> {
    Command::new("post")
        .about("Manage gemlog posts")
        .long_about(
"A post is just a page residing in the \"gemlog\" subdirectory, which gets indexed
and included in feeds. Posts must be published before they will appear in the
generated capsule, and will appear in reverse chronoogical order. Posts, like all
pages, may also be categorized using tags, and a page will be auto generated for
every tag in the capsule with links to every page and gemlog post which includes
that tag."
        )
        .arg(
            Arg::new("title")
                .help("The title of the post")
                .takes_value(true)
                .multiple_values(false),
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
                        .required(false),
                )
                .arg(
                    Arg::new("tags")
                        .help("Tags for this post (optional)")
                        .short('t')
                        .long("tags")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(false),
                ),
        )
        .subcommand(Command::new("publish").about("Marks the post as published"))
        .subcommand(Command::new("edit").about("Opens the post in an editor"))
}

pub fn build_page() -> Command<'static> {
    Command::new("page")
        .about("Manage pages")
        .long_about(
"Pages must be published before they will appear in the generated capsule. Pages
may also be categorized using tags, and a page will be auto generated for every
tag in the capsule with links to every page and gemlog post which includes that
tag. The special page \"index.gmi\", which is automatically generated when the
capsule is first generated, will also display a configurable number of gemlog
post links wherever the string \"{% posts %}\" is placed within it's content
section."
        )
        .arg(
            Arg::new("title")
                .help("The title of the page")
                .short('t')
                .long("title")
                .takes_value(true)
                .multiple_values(false),
        )
        .arg(
            Arg::new("path")
                .help("Path to the page")
                .short('p')
                .long("path")
                .takes_value(true)
                .multiple_values(false),
        )
        .group(
            ArgGroup::new("specifier")
                .required(true)
                .args(&["title", "path"])
                .multiple(true),
        )
        .subcommand(
            Command::new("init")
                .about("Initializes a new page")
                .arg(
                    Arg::new("summary")
                        .help("A short summary of the page (optional)")
                        .short('s')
                        .long("summary")
                        .takes_value(true)
                        .multiple_values(false)
                        .required(false),
                )
                .arg(
                    Arg::new("tags")
                        .help("Tags for this page (optional)")
                        .short('t')
                        .long("tags")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(false),
                ),
        )
        .subcommand(Command::new("publish").about("Marks the page as published"))
        .subcommand(Command::new("edit").about("Opens the page in an editor"))
}

/// Generates the command line options
pub fn build() -> Command<'static> {
    Command::new("zond")
        .about("A static Gemini capsule generator")
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(build_init())
        .subcommand(build_build())
        .subcommand(build_post())
        .subcommand(build_page())
}
