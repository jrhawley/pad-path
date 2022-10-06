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
cargo install --path .
```

## Usage

```shell
Intuitively modify your `$PATH`

USAGE:
    pad [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Add a directory
    clean     Remove duplicates and non-existent directories [aliases: dedup]
    dn        Decrease priority for a directory [aliases: down, dec]
    help      Prints this message or the help of the given subcommand(s)
    ls        List the directories in PATH [aliases: echo]
    revert    Revert to a previous version of PATH [aliases: undo]
    rm        Remove a directory [aliases: del]
    up        Increase priority for a directory [aliases: inc]
```

It would be convenient, but insecure, to have `pad-path` modify the shell's environment variables directly.
Instead, `pad-path` prints out what the new `$PATH` will look like, and the user can set it as the value for `$PATH`, or pipe it to another command.

### Examples

In Bash, an example workflow is below.

```bash
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

Similarly, in PowerShell, you can assign the output of a `pad-path` command to `$Env:Path`.

```powershell
# check the value of $Env:Path
> pad ls
C:\WINDOWS\system32
C:\WINDOWS
C:\WINDOWS\System32\WindowsPowerShell\v1.0
C:\WINDOWS\System32\OpenSSH
C:\Program Files\dotnet

# add a new variable to $Env:Path
> $Env:Path = (pad add "C:\Program Files\new_dir")

# check that the folder has been added correctly
> pad ls
C:\WINDOWS\system32
C:\WINDOWS
C:\WINDOWS\System32\WindowsPowerShell\v1.0
C:\WINDOWS\System32\OpenSSH
C:\Program Files\dotnet
C:\Program Files\new_dir
```

## Development

### Compilation

`pad-path` is designed to work on Windows, macOS, and Linux operating systems.
To compile binaries for each of these systems, we make use of [`cargo-make`](https://sagiegurari.github.io/cargo-make/).

To build a release version, run:

```shell
# for the OS you're working on
cargo make build-release

# for a specific OS
cargo make build-release-windows
cargo make build-release-macos
cargo make build-release-linux

# for all OS's
cargo make build-release-all
```

To create a tag and a release on GitHub, we make use of the [GitHub CLI](https://cli.github.com/).

```shell
# for the OS you're working on
cargo make release

# for a specific OS
cargo make release-windows
cargo make release-macos
cargo make release-linux

# for all OS's
cargo make release-all
```

### Testing

We generally follow the testing ideas in [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/ch11-00-testing.html).
To run tests quickly and concisely, we make use of [`cargo-nextest`](https://nexte.st/).

```shell
# run all tests
cargo nextest run

# or, equivalently
cargo make test
```
