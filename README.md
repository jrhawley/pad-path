# pad

Intuitively modify your `$PATH`.

![Crates.io](https://img.shields.io/crates/v/pad-path)

## Installation

### Cargo

```shell
cargo install pad-path
```

### Build from source

```shell
git clone https://github.com/jrhawley/pad-path
cd pad-path
cargo build --release
```

## Usage

```shell
pad-path 0.1.5
Intuitively modify your `$PATH`

USAGE:
    pad [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Add a directory
    clean    Remove duplicates and non-existent directories [aliases: dedup]
    dn       Decrease priority for a directory [aliases: down, dec]
    help     Prints this message or the help of the given subcommand(s)
    ls       List the directories in PATH [aliases: echo]
    rm       Remove a directory [aliases: del]
    up       Increase priority for a directory [aliases: inc]
```

It would be convenient, but insecure, to have `pad-path` modify the shell's environment variables directly.
Instead, `pad-path` prints out what the new `$PATH` will look like, and the user can set it as the value for `$PATH`, or pipe it to another command.

An example workflow is below.

```shell
# display your current $PATH
> pad ls
/folder/to/remove
/usr/local/sbin
/usr/local/bin
/usr/sbin
/usr/bin
/sbin
/bin
/usr/games
/usr/local/games
/snap/bin

# preview what $PATH would look like if you remove the first folder
> pad rm /folder/to/remove
/usr/local/sbin
/usr/local/bin
/usr/sbin
/usr/bin
/sbin
/bin
/usr/games
/usr/local/games
/snap/bin

# set the new $PATH
> export PATH=$(pad rm /folder/to/remove)

# see that the new path is set
> echo $PATH
/usr/local/sbin
/usr/local/bin
/usr/sbin
/usr/bin
/sbin
/bin
/usr/games
/usr/local/games
/snap/bin
```
