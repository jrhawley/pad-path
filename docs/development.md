# Development

## Compilation

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

To create a tag and a release on GitLab, we make use of the [GitLab CLI](https://gitlab.com/gitlab-org/cli).

## Testing

We generally follow the testing ideas in [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/ch11-00-testing.html).
To run tests quickly and concisely, we make use of [`cargo-nextest`](https://nexte.st/).

```shell
# run all tests
cargo nextest run

# or, equivalently
cargo make test
```

