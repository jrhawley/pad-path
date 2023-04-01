# Changelog

## [0.2.6] - 2023-04-01

### Added

- [Nix flake](https://nixos.wiki/wiki/Flakes) for compilation and development environments

### Fixed

- Tests in the `crate::path::clean` module now pass on Linux systems that don't have `/usr/` as a top-level directory
  - This was discovered when trying to build a Nix package

## [0.2.5] - 2022-10-05

### Fixed

- Testing on Arch Linux no longer fails due to a symlink in `/usr/lib/` (see [#1](https://github.com/jrhawley/pad-path/pull/1), thanks [@thecakeisalie25](https://github.com/thecakeisalie25))
- Removed erroneous `echo` alias for the `revert` subcommand

### Changed

- Upgraded from `home` to `dirs-next` crate to follow the [XDG Base Directories](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html) and [XDG User Directories](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications
- Upgraded to `clap` v4

## [0.2.4] - 2022-08-30

### Changed

- Switched from `structopt` to `clap` v3

## [0.2.3] - 2021-12-09

### Fixed

- Prepending a duplicate directory with `pad add` no longer leaves two copies of the directory in `$PATH` when forced.
- Removing a directory not found in `$PATH` properly raises a warning.
- Other errors that should have been raised are now raising correctly.

## [0.2.2] - 2021-11-19

### Added

- Quiet `-q` CLI argument to suppress errors or warnings.

### Changed

- Switched from CLI parsing with `clap` to `structopt`.
  - Everything should stil work normally, but there may be errors that I haven't tested for, yet.

## [0.2.1] - 2021-11-16

### Added

- Documentation throughout the crate.

### Changed

- Using the 2021 edition of Rust instead of the 2018 edition.

### Fixed

- Error with Linux and macOS systems when looking for trailing slashes in directories.

## [0.2.0] - 2021-09-08

- Adding revert/undo functionality
  - This makes use of a local config file that contains all the path history

## [0.1.10] - 2021-04-15

- Simplified error messages when a directory doesn't exist

## [0.1.9] - 202Y-MM-DD

- Bugfix: `pad add -p` no longer erases `$PATH`

## [0.1.8] - 2021-03-31

- Safe failure mode by always printing the current `$PATH` if an error occurs

## [0.1.7] - 202Y-03-31

- Automatically clean `$PATH`
- Allow for adding multiple directories in a single command

## [0.1.6] - 2021-03-30

- Consistent environment variable behaviour on Windows machines as other operating systems

## [0.1.5] - 2021-03-30

- Deprecating `undo` subcommand due to insecure and lack of persistence trying to directly edit environment variables

## [0.1.4] - 2020-11-16

- Bugfixes for *nix operating systems
- Option to show `OLD_PATH` with `ls` subcommand

## [0.1.3] - 2020-11-16

- Additionally remove non-existing directories from `$PATH` with `clean` subcommand
- Default to +/-1 positions for `up` and `dn` subcommands

## [0.1.2] - 2020-11-16

- Making use of `OLD_PATH` directory for safer changes
- Adding `undo` subcommand to safely revert back to the `OLD_PATH`

## [0.1.1] - 2020-11-16

`clean` subcommand to remove duplicate directories from `$PATH`.

## [0.1.0] - 2020-11-06

Initial release.
Can add, remove, list, and change priority of directories in `$PATH`.
