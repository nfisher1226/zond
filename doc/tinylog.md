# The Tinylog
A [Tinylog](https://codeberg.org/bacardi55/gemini-tinylog-rfc) is like a microblog,
containing shorted posts which all appear on a single page.

To add an entry to your Tinylog, call zond with the `tinylog` subcommand and no
other arguments. An editor will open with a blank file where you can write your post.
Saving the file and closing the editor will update the Tinlylog accordingly. If the
post is empty, the log will not be updated.

The log may have a header as well. You can edit the log manually by passing the '--edit'
or '-e' flag to `zond tinylog`.

Next: Building the capsule](build.md)
