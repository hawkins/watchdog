extern crate clap;
extern crate notify;
#[macro_use]
extern crate shell;

use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;

use clap::{App, Arg};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

fn watch(path: &str, command: &str) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

    try!(watcher.watch(path, RecursiveMode::Recursive));

    loop {
        match rx.recv() {
            Ok(event) => {
                cmd!(command).run();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = App::new("Watchdog")
        .version("0.1")
        .author("Josh Hawkins <hawkins@users.noreply.github.com>")
        .about("Watches the filesystem for changes and runs tasks in response")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Path used for matching files")
                .value_name("FILE/FOLDER")
                .takes_value(true)
                .required(true),
        ).arg(
            Arg::with_name("COMMAND")
                .help("Command ran on response to changes")
                .index(1)
                .required(true),
        ).get_matches();

    if let Err(e) = watch(
        matches.value_of("path").unwrap(),
        matches.value_of("COMMAND").unwrap(),
    ) {
        println!("error: {:?}", e)
    }
}
