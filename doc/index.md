# Usage - Zond
## Initializing a new capsule
```sh
zond init
```
The `init` subcommand can optionally take a number of arguments to preset some
variables in `Config.ron`. To see the full list, run:
```sh
zond help init
# Shell output
zond-init
Initialize a new capsule

USAGE:
    zond init [OPTIONS]

    OPTIONS:
        -a, --author <author>      The principle author of this capsule
        -d, --domain <domain>      The domain serving this capsule
        -e, --entries <entries>    Number of gemlog entries to display links for on the homepage
        -f, --feed <feed>          The type of feed to generate. Atom, Gemini, of Both
        -h, --help                 Print help information
        -l, --license <license>    Commons license to use. One of CcBy, CcBySa, CcByNc, CcByNcSa,
                                   CcByNd, CcByNcNd. For information on Creative Commons licenses, see
                                   https://creativecommons.org/about/cclicenses/
        -m, --email <email>        The email address of the principle author
        -p, --path <path>          The path from the server root to this capsule
        -s, --show_email           Add a link to the author's email on each page
        -t, --title <title>        The title of this caspule
        -u, --url <url>            The principle author's homepage
```
### An example `Config.ron`
```
(
    title: "The Sabbath diaries",
    author: (
        name: "John Osborne",
        email: Some("oz@black.sabbath.fm"),
        url: Some("black.sabbath.fm"),
    ),
    domain: "black.sabbath.fm",
    path: Some("~john"),
    entries: 3,
    feed: Some(Both),
    license: Some(CcBySa),
    show_email: true,
    footer_links: [(url: "spartan://black.sabbath.fm", display: "Spartan site")],
)
```
Next: [Working with pages](page.md)
