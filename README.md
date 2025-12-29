# pad-path

Intuitively modify your `$PATH`.

**Note**: this repository has been archived.
Development of this project has switched to [this location](https://gitlab.com/jrhawley/pad-path).

![Crates.io](https://img.shields.io/crates/v/pad-path)

## Installation

### Windows (via Scoop)

```shell
# add the bucket containing pad-path's manifest
scoop bucket add jrhawley https://gitlab.com/jrhawley/scoop-bucket

# install
scoop install pad-path
```

### Nix

```shell
nix profile install gitlab:jrhawley/pad-path
```

### Build from source (via Cargo)

```shell
# directly from crates.io
cargo install pad-path

# or, equivalently, after downloading the code repo
git clone https://gitlab.com/jrhawley/pad-path
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
