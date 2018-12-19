# Watchdog

⚠️ Watch filesystem for changes and then run a command

Great for automatically running `make test` or similar commands in response to file changes.

## Usage

TODO: This isn't stable yet, but here's the output for `--help` at 0.2:

```
USAGE:
    watchdog [FLAGS] [OPTIONS] <COMMAND> [-- <PATH>...]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enables verbose output

OPTIONS:
    -g, --glob <GLOB>    Glob used for matching files

ARGS:
    <COMMAND>    Command ran on response to changes
    <PATH>...    File path(s) used for matching files
```

## Goals

- Easy, inuitive way to select files to watch
  - [ ] Regular expressions (#1)
  - [x] Globs
    - Both via your shell and via Rust internals, pick your poison!
  - [x] Explicit file paths
- [ ] Sensible GNU `make` interop (#4)
- Simple, out-of-the-way API
  - It's a simple problem. Therefore, keep the solution simple, too, stupid.
