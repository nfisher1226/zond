# Vostok
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
In addition to outputting a binary at `<cwd>/target/release/vostok`, there will be
completion scripts generated for the Bash, Zsh, Fish and Powershell scripts. See
the terminal output for where they are located, and your shell's documentation for
where to place them.

## Usage
For full usage information, see the documents in the `doc/` subdirectory of the
source repository.
