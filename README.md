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
pad-path 0.1.3
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
    undo     Undo most recent changes to PATH [aliases: revert]
    up       Increase priority for a directory [aliases: inc]
```
