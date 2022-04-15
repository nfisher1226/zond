# Working with gemlog posts - Zond
A gemlog post is like an ordinary page, with the following exceptions.
* It will always be in the `gemlog` subdirectory
* The `post` subcommand has no `--path` option
* The post will appear in the gemlog index, and a few of the most recent posts
  will appear on the capsule's main index page automatically
* The post will have all of the same footer content plus a link to the gemlog
  index
Everything that can be done from Zond for an ordinary page using the `page`
subcommand can also be done to a post using the `post` subcommand (excepting the
`--path` argument).

To specify the number of posts which will appear in the main index, set the
`entries` field as desired in the capsule's `Config.ron` file.

Next: [Building the capsule](build.md)
