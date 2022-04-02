# Usage
## Initializing a new capsule
```sh
zond init
```
The `init` subcommand can optionally take a number of arguments to preset some
variables in `Config.ron`. To see the full list, run:
```sh
zond help init
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
)
```
