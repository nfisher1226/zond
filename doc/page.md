# Working with pages - Zond
### Creating a new page
The first step in working with a page is creating it.
```sh
# Specifying the page to create by path
zond page --path content/songs/the_writ.gmi init
# Specifying the page to create by title
zond page --title "The Writ" init
# Note: if specified by title without a path the page will be in the site root
# Seeing all options
zond help page init
# A more advanced example
zond page -p content/songs/the_writ.gmi --summary "An epic bombast from their 1976 album Sabotage" \
    --tags songs --tags favorites --edit
```

Pages must be published before they will be included in the generated output when
running `zond build`. Before publication, a page's frontmatter might appear as so:
```Rust
(
    title: "Iron Man",
    summary: Some("An early classic with staying power"),
    published: None,
    tags: ["songs", "popular"],
)
---
```
After publication, the `published` field will have a date and time associated with
it. To publish a page, use the `publish` subcommand.
```sh
zond page -p content/songs/iron_man.gmi publish
```
### Editing pages
To edit a page, one can either just open the page in a text editor, use the `edit`
subcommand, or the `--edit` flag when creating the page. Page content is just
ordinary gemtext. However, the page title will automatically be inserted at the
top of the page and certain content will appear at the bottom of every page in
the capsule, such as a link to the capsule root and copyright information. This
makes it easier to maintain consistency accross the entire site with less book
keeping.

Next: [Working with gemlog posts](post.md)
