use {
    clap::{value_parser, Arg, ArgAction, ArgGroup, Command, ValueHint},
    gettextrs::gettext,
};

#[must_use]
/// The init subcommand
pub fn init() -> Command {
    Command::new("init")
        .about(gettext("Initialize a new capsule"))
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
                .help(gettext("The principle author of this capsule"))
                .num_args(1)
                .required(false),
             Arg::new("email")
                .short('m')
                .long("email")
                .value_hint(ValueHint::EmailAddress)
                .help(gettext("The email address of the principle author"))
                .num_args(1)
                .required(false),
             Arg::new("url")
                .short('u')
                .long("url")
                .value_hint(ValueHint::Url)
                .help(gettext("The principle author's homepage"))
                .num_args(1)
                .required(false),
             Arg::new("domain")
                .short('d')
                .long("domain")
                .value_hint(ValueHint::Hostname)
                .help(gettext("The domain serving this capsule"))
                .num_args(1)
                .required(false),
             Arg::new("path")
                .short('p')
                .long("path")
                .help(gettext("The path from the server root to this capsule"))
                .num_args(1)
                .required(false),
             Arg::new("entries")
                .short('e')
                .long("entries")
                .value_parser(value_parser!(usize))
                .default_value("3")
                .help(gettext("Number of gemlog entries to display links for on the homepage"))
                .num_args(1)
                .required(false),
            Arg::new("display_date")
                .short('D')
                .long("display_date")
                .help(
                    format!(
                        "{} (always|gemlog|never)",
                        gettext("Which pages to display the publication date under the title")
                    )
                )
                .num_args(1)
                .value_parser(["always", "gemlog", "never"])
                .required(false),
             Arg::new("feed")
                .short('f')
                .long("feed")
                .value_parser(["atom", "gemini", "both"])
                .help(gettext("The type of feed to generate. Atom, Gemini, or Both"))
                .num_args(1)
                .required(false),
             Arg::new("license")
                .short('l')
                .long("license")
                .value_parser(["CcBy", "CcBySa", "CcByNc", "CcByNd", "CcByNcNd"])
                .help(gettext("Commons license to use. One of CcBy, CcBySa, CcByNc, CcByNcSa, CcByNd, CcByNcNd. For information on Creative Commons licenses, see https://creativecommons.org/about/cclicenses/"))
                .num_args(1)
                .required(false),
             Arg::new("show_email")
                .short('s')
                .long("show_email")
                .action(ArgAction::SetTrue)
                .help(gettext("Add a link to the author's email on each page"))
                .action(clap::ArgAction::SetTrue)
                .required(false),
             ])
}

#[must_use]
/// The build subcommand
pub fn build() -> Command {
    Command::new("build")
        .about(gettext("Build the capsule"))
        .visible_alias("bld")
        .visible_short_flag_alias('b')
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help(gettext("The location to output the generated capsule"))
                .num_args(1)
                .required(false),
        )
}

#[must_use]
/// The post init subcommand
pub fn post_init() -> Command {
    Command::new("init")
        .about(gettext("Initializes a new post"))
        .visible_alias("in")
        .visible_short_flag_alias('i')
        .args([
            Arg::new("summary")
                .help(gettext("A short summary of the post (optional)"))
                .short('s')
                .long("summary")
                .num_args(1)
                .required(false),
            Arg::new("tags")
                .help(gettext("Tags for this post (optional)"))
                .short('t')
                .long("tags")
                .num_args(1..)
                .value_delimiter(',')
                .required(false),
            Arg::new("edit")
                .help(gettext("Edit the newly created post"))
                .short('e')
                .long("edit")
                .action(ArgAction::SetTrue)
                .required(false),
            Arg::new("publish")
                .help("Publish the newly created post after editing. Implies `-e`")
                .short('p')
                .long("publish")
                .action(ArgAction::SetTrue)
                .required(false),
        ])
}

#[must_use]
/// the post subcommand
pub fn post() -> Command {
    Command::new("post")
        .about(gettext("Manage gemlog posts"))
        .long_about(gettext(
            "A post is just a page residing in the \"gemlog\" subdirectory, which gets indexed
and included in feeds. Posts must be published before they will appear in the
generated capsule, and will appear in reverse chronoogical order. Posts, like all
pages, may also be categorized using tags, and a page will be auto generated for
every tag in the capsule with links to every page and gemlog post which includes
that tag.",
        ))
        .visible_alias("po")
        .arg(
            Arg::new("title")
                .help(gettext("The title of the post"))
                .num_args(1),
        )
        .subcommands([
            post_init(),
            Command::new("publish")
                .about(gettext("Marks the post as published"))
                .visible_alias("pub"),
            Command::new("edit")
                .about(gettext("Opens the post in an editor"))
                .visible_alias("ed")
                .visible_short_flag_alias('e'),
        ])
}

#[must_use]
/// The page init subcommand
pub fn page_init() -> Command {
    Command::new("init")
        .about(gettext("Initializes a new page"))
        .visible_alias("in")
        .visible_short_flag_alias('i')
        .args([
            Arg::new("summary")
                .help(gettext("A short summary of the page (optional)"))
                .short('s')
                .long("summary")
                .num_args(1)
                .required(false),
            Arg::new("tags")
                .help(gettext("Tags for this page (optional)"))
                .short('t')
                .long("tags")
                .num_args(1..)
                .value_delimiter(',')
                .required(false),
            Arg::new("edit")
                .help(gettext("Edit the newly created page"))
                .short('e')
                .long("edit")
                .action(ArgAction::SetTrue)
                .required(false),
            Arg::new("publish")
                .help("Publish the newly created page after editing. Implies `-e`")
                .short('p')
                .long("publish")
                .action(ArgAction::SetTrue)
                .required(false),
        ])
}

#[must_use]
/// The page subcommand
pub fn page() -> Command {
    Command::new("page")
        .about(gettext("Manage pages"))
        .long_about(gettext(
            "Pages must be published before they will appear in the generated capsule. Pages
may also be categorized using tags, and a page will be auto generated for every
tag in the capsule with links to every page and gemlog post which includes that
tag. The special page \"index.gmi\", which is automatically generated when the
capsule is first generated, will also display a configurable number of gemlog
post links wherever the string \"{% posts %}\" is placed within it's content
section.",
        ))
        .visible_alias("pg")
        .args([
            Arg::new("title")
                .help(gettext("The title of the page"))
                .short('t')
                .long("title")
                .num_args(1),
            Arg::new("path")
                .help(gettext("Path to the page"))
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
                .about(gettext("Marks the page as published"))
                .visible_alias("pub"),
            Command::new("edit")
                .about(gettext("Opens the page in an editor"))
                .visible_short_flag_alias('e')
                .visible_alias("ed"),
        ])
}

#[must_use]
pub fn tinylog() -> Command {
    Command::new("tinylog")
        .about("Creates a new tinylog entry")
        .long_about(
            "A tinlylog is a single file containing multiple short log \
            entries. If a quoted string is passed to this command, that string \
            will be appended to the tinylog as a new entry. If the `edit` flag \
            is passed, the log file will be opened in an editor. If this command \
            is called with no arguments an editor will be opened to create a new \
            tinylog entry.",
        )
        .visible_alias("tl")
        .args([
            Arg::new("post")
                .help("The full text of the tinylog entry")
                .num_args(1),
            Arg::new("tags")
                .help("Adds one or more tags to the tinylog")
                .short('t')
                .long("tags")
                .num_args(1..),
            Arg::new("edit")
                .help("Edit the tinlylog in your editor")
                .short('e')
                .long("edit")
                .conflicts_with_all(["post", "tags"])
                .action(ArgAction::SetTrue),
        ])
}

#[must_use]
/// Generates the command line options
pub fn zond() -> Command {
    Command::new("zond")
        .about(gettext("A static Gemini capsule generator"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([init(), build(), post(), page(), tinylog()])
}
