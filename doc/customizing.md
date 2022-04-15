# Customizing your capsule
A number of things can be customized in your capsule by editing the `Config.ron`
file. Here's an example.
```Rust
(
    title: "The Sabbath diaries",
    author: (
        name: "John Osborne",
        email: Some("oz@black.sabbath.fm"),
        url: Some("black.sabbath.fm"),
    ),
    domain: "black.sabbath.fm",
    path: None,
    entries: 3,
    feed: Some(Both),
    license: Some(CcBySa),
    show_email: true,
)
```
### What the fields affect
* title - This is the title of the capsule as it will appear in the main index and
  in the feeds
* author - who is the principle author of this capsule
  * name - the author's name (required)
  * email - the author's email (optional)
  * url - the author's homepage (optional)
* domain - the domain which will serve this capsule
* path - the path from the domain root to this capsule. This is useful for a shared
  hosting setup where each user's capsule shares a domain but appears in a
  subdirectory, such as `gemini://example.com/~johndoe/`
* entries - the number of entries which will appear on the main capsule index. These
  entries will appear wherever in the index the string `{% posts %}` appears in
  the index source (content/index.gmi). If the string is left off entirely then
  no posts will be displayed.
* feed - which feeds to generate. This can be Some(Gemini), Some(Atom), Some(Both),
  or None.
* license - For no license, put `None` here. Recognized licenses are as laid out
  by [creative commons](https://creativecommons.org/), although any alternative
  license can be used by specifying `Other` and providing a string.
  * Some(CcBy) - by author
  * Some(CcBySa) - by author share alike
  * Some(CcByNc) - by author non-commercial
  * Some(CcByNcSa) - by author non-commercial share alike
  * Some(CcByNd) - by author no derivatives or adaptations
  * Some(CcByNcNd) - by author non-commercial no derivatives or adaptations
  * Some(CcZero) - dedicated to public domain
  * Some(Other("My License")) - any custom license
  * None - no license specified
* show_email - whether or not to include a link to the author's email on each
  page. Requires an email to be set in the `author: email` field.

### Using a custom ascii art banner
Any text placed in the file "content.txt" will be included in a preformatted block
at the beginning of every page.

### Including other files
Any other files inside the `content` directory will be copied over to a corresponding
location in `public` (or the path specified by `zond build --output`). Thus, a png
image placed at `content/gemlog/ozzy.png` will be copied over to
`public/gemlog/ozzy.png` when the capsule is built. This applies to every file
which does not have a .gmi extension.

### Further reading
The rust api docs can be generated if desired by running `cargo doc` from within
the zond source directory.
