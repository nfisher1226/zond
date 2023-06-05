# Zond
Zond (Russian: Зонд, lit. 'probe') was the name given to two distinct series of
Soviet robotic spacecraft launched between 1964 and 1970. The first series, based
on the 3MV planetary probe, was intended to gather information about nearby
planets. The second series of test spacecraft was intended as a precursor to
remote-controlled robotic circumlunar loop flights, using a stripped-down variant
of Soyuz spacecraft, consisting of the service and descent modules, but lacking
the orbital module. Two tortoises and other lifeforms aboard Zond 5 were the first
terrestrial organisms to travel around the Moon and return to Earth.

In this context, Zond is a static gemini capsule generator. Think static site
generator like Hugo or Jekyll but for [Project Gemini](https://gemini.circumlunar.space).

## Building
```sh
cargo build --release
```
If desired, a release distribution can then be generated which includes the following:
- Shell completions for bash, fish, powershell and zsh
- Unix man pages
- Documentation
- Translations
```sh
cargo build --release --features=bootstrap
```
This will create a second binary named `bootstrap` which can then be used to either
install the files directly into the filesystem or install everything into a staging
directory, ready for creating a distribution package. For details, run the `bootstrap`
binary with the `--help` option.
## Usage
For full usage information, see the documents in the `doc/` subdirectory of the
source repository. Short help can also be had via the `--help` or `-h` options passed
to the main command or any subcommand, or by viewing one of the generated Unix manual
pages.
