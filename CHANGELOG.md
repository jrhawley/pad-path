# Changelog

## v0.1.10

- Simplified error messages when a directory doesn't exist

## v0.1.9

- Bugfix: `pad add -p` no longer erases `PATH`

## v0.1.8

- Safe failure mode by always printing the current `PATH` if an error occurs

## v0.1.7

- Automatically clean `PATH`
- Allow for adding multiple directories in a single command

## v0.1.6

- Consistent environment variable behaviour on Windows machines as other operating systems

## v0.1.5

- Deprecating `undo` subcommand due to insecure and lack of persistence trying to directly edit environment variables

## v0.1.4

- Bugfixes for *nix operating systems
- Option to show `OLD_PATH` with `ls` subcommand

## v0.1.3

- Additionally remove non-existing directories from `PATH` with `clean` subcommand
- Default to +/-1 positions for `up` and `dn` subcommands

## v0.1.2

- Making use of `OLD_PATH` directory for safer changes
- Adding `undo` subcommand to safely revert back to the `OLD_PATH`

## v0.1.1

`clean` subcommand to remove duplicate directories from `PATH`.

## v0.1.0

Initial release.
Can add, remove, list, and change priority of directories in `PATH`.
