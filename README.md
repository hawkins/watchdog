# Watchdog

⚠️ Watch filesystem for changes and then run a command

Great for automatically running `make test` or similar commands in response to file changes.

## Usage

TODO: This isn't stable yet, but here's the output for `--help` at 0.1:

```
USAGE:
    watchdog [FLAGS] <COMMAND> --path <FILE/FOLDER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enables verbose output

OPTIONS:
    -p, --path <FILE/FOLDER>    Path used for matching files

ARGS:
    <COMMAND>    Command ran on response to changes
```

## Goals

- [ ] Pattern match files to watch via regular expressions (#1)
- [ ] Sensible Make interop (#4)
- Simple, out-of-the-way API
  - It's a simple problem. Therefore, keep the solution simple, too, stupid.
