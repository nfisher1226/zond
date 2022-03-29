# Vostok
Running concurrently to NASA's project Mercury, the Soviet *Vostok*, or if you
prefer the original Russian, *Восток* project was a human spaceflight program
with the goal of putting the first human into low earth orbit and returning them
safely. Yuri Gagarin in fact became the first human to do so on April 12th, 1961.

In this context, Vostok is a static gemini capsule generator, think static site
generator like Hugo or Jekyll but for [Project Gemini](https://gemini.circumlunar.space).

## Building
```sh
cargo build --release
```
In addition to outputting a binary at `<cwd>/target/release/vostok`, there will be
completion scripts generated for the Bash, Zsh, Fish and Powershell scripts. See
the terminal output for where they are located, and your shell's documentation for
where to place them.

## Usage
### Initialize a new capsule in the current directory
```sh
vostok init
```
This will generate a `Config.ron` file, which can then be edited to suit the needs
of the capsule. Alternatively, many of the capsule parameters may be set by
supplying arguments to the `init` subcommand. For the full list of arguments, run
this command:
```sh
vostok help init
```
