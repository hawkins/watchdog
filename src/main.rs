extern crate clap;
extern crate notify;
extern crate subprocess;

use std::sync::mpsc::channel;
use std::time::Duration;

use clap::{App, Arg, ArgMatches};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use subprocess::Exec;

fn exec(m: &ArgMatches) {
    let command = m.value_of("COMMAND").unwrap();
    Exec::shell(command).join().expect("Failed");
}

fn watch(m: &clap::ArgMatches) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

    try!(watcher.watch(m.value_of("path").unwrap(), RecursiveMode::Recursive));

    loop {
        match rx.recv() {
            Ok(event) => {
                if m.is_present("verbose") {
                    println!("{:?}", event);
                }

                match event {
                    notify::DebouncedEvent::Create { .. } => exec(m),
                    notify::DebouncedEvent::Write { .. } => exec(m),
                    notify::DebouncedEvent::Remove { .. } => exec(m),
                    notify::DebouncedEvent::Rename { .. } => exec(m),
                    _ => (),
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
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
        ).arg(
            Arg::with_name("verbose")
                .help("Enables verbose output")
                .short("v")
                .long("verbose"),
        ).get_matches();

    if let Err(e) = watch(&matches) {
        println!("error: {:?}", e)
    }
}
