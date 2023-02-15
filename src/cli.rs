use clap::{Arg, ArgGroup, Command, ValueHint, value_parser};

#[must_use]
/// The init subcommand
pub fn init() -> Command {
    Command::new("init")
        .about("Initialize a new capsule")
        .visible_alias("in")
        .visible_short_flag_alias('i')
        .args([
             Arg::new("title")
                .short('t')
                .long("title")
                .help("The title of this caspule")
                .num_args(1)
                .required(false),
             Arg::new("author")
                .short('a')
                .long("author")
                .help("The principle author of this capsule")
                .num_args(1)
                .required(false),
             Arg::new("email")
                .short('m')
                .long("email")
                .help("The email address of the principle author")
                .value_hint(ValueHint::EmailAddress)
                .num_args(1)
                .required(false),
             Arg::new("url")
                .short('u')
                .long("url")
                .help("The principle author's homepage")
                .value_hint(ValueHint::Url)
                .num_args(1)
                .required(false),
             Arg::new("domain")
                .short('d')
                .long("domain")
                .help("The domain serving this capsule")
                .value_hint(ValueHint::Hostname)
                .num_args(1)
                .required(false),
             Arg::new("path")
                .short('p')
                .long("path")
                .help("The path from the server root to this capsule")
                .num_args(1)
                .required(false),
             Arg::new("entries")
                .short('e')
                .long("entries")
                .help("Number of gemlog entries to display links for on the homepage")
                .value_parser(value_parser!(usize))
                .default_value("3")
                .num_args(1)
                .required(false),
            Arg::new("display_date")
                .short('D')
                .long("display_date")
                .help("Which pages to display the publication date under the title")
                .num_args(1)
                .value_parser(["always", "gemlog", "never"])
                .required(false),
             Arg::new("feed")
                .short('f')
                .long("feed")
                .help("The type of feed to generate.")
                .value_parser(["atom", "gemini", "both"])
                .num_args(1)
                .required(false),
             Arg::new("license")
                .short('l')
                .long("license")
                .help("Commons license to use. For information on Creative Commons licenses, see https://creativecommons.org/about/cclicenses/")
                .value_parser(["CcBy", "CcBySa", "CcByNc", "CcByNd", "CcByNcNd"])
                .num_args(1)
                .required(false),
             Arg::new("show_email")
                .short('s')
                .long("show_email")
                .help("Add a link to the author's email on each page")
                .action(clap::ArgAction::SetTrue)
                .required(false),
             ])
}

#[must_use]
/// The build subcommand
pub fn build() -> Command {
    Command::new("build")
        .about("Build the capsule")
        .visible_alias("bld")
        .visible_short_flag_alias('b')
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("The location to output the generated capsule")
                .num_args(1)
                .required(false),
        )
}

#[must_use]
/// The post init subcommand
pub fn post_init() -> Command {
    Command::new("init")
        .about("Initializes a new post")
        .visible_alias("in")
        .visible_short_flag_alias('i')
        .args([
            Arg::new("summary")
                .help("A short summary of the post (optional)")
                .short('s')
                .long("summary")
                .num_args(1)
                .required(false),
            Arg::new("tags")
                .help("Tags for this post (optional)")
                .short('t')
                .long("tags")
                .num_args(1..)
                .value_delimiter(',')
                .required(false),
            Arg::new("edit")
                .help("Edit the newly created post")
                .short('e')
                .long("edit")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        ])
}

#[must_use]
/// the post subcommand
pub fn post() -> Command {
    Command::new("post")
        .about("Manage gemlog posts")
        .long_about(
            "A post is just a page residing in the \"gemlog\" subdirectory, which gets indexed
and included in feeds. Posts must be published before they will appear in the
generated capsule, and will appear in reverse chronoogical order. Posts, like all
pages, may also be categorized using tags, and a page will be auto generated for
every tag in the capsule with links to every page and gemlog post which includes
that tag.",
        )
        .visible_alias("po")
        .arg(Arg::new("title").help("The title of the post").num_args(1))
        .subcommands([
            post_init(),
            Command::new("publish")
                .about("Marks the post as published")
                .visible_alias("pub"),
            Command::new("edit")
                .about("Opens the post in an editor")
                .visible_alias("ed")
                .visible_short_flag_alias('e'),
        ])
}

#[must_use]
/// The page init subcommand
pub fn page_init() -> Command {
    Command::new("init")
        .about("Initializes a new page")
        .visible_alias("in")
        .visible_short_flag_alias('i')
        .args([
            Arg::new("summary")
                .help("A short summary of the page (optional)")
                .short('s')
                .long("summary")
                .num_args(1)
                .required(false),
            Arg::new("tags")
                .help("Tags for this page (optional)")
                .short('t')
                .long("tags")
                .num_args(1..)
                .value_delimiter(',')
                .required(false),
            Arg::new("edit")
                .help("Edit the newly created page")
                .short('e')
                .long("edit")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        ])
}

#[must_use]
/// The page subcommand
pub fn page() -> Command {
    Command::new("page")
        .about("Manage pages")
        .long_about(
            "Pages must be published before they will appear in the generated capsule. Pages
may also be categorized using tags, and a page will be auto generated for every
tag in the capsule with links to every page and gemlog post which includes that
tag. The special page \"index.gmi\", which is automatically generated when the
capsule is first generated, will also display a configurable number of gemlog
post links wherever the string \"{% posts %}\" is placed within it's content
section.",
        )
        .visible_alias("pg")
        .args([
            Arg::new("title")
                .help("The title of the page")
                .short('t')
                .long("title")
                .num_args(1),
            Arg::new("path")
                .help("Path to the page")
                .short('p')
                .long("path")
                .num_args(1),
        ])
        .group(
            ArgGroup::new("specifier")
                .required(true)
                .args(["title", "path"])
                .multiple(true),
        )
        .subcommands([
            page_init(),
            Command::new("publish")
                .about("Marks the page as published")
                .visible_alias("pub"),
            Command::new("edit")
                .about("Opens the page in an editor")
                .visible_short_flag_alias('e')
                .visible_alias("ed"),
        ])
}

#[must_use]
/// Generates the command line options
pub fn zond() -> Command {
    Command::new("zond")
        .about("A static Gemini capsule generator")
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([init(), build(), post(), page()])
}
