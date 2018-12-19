extern crate clap;
extern crate globset;
extern crate notify;
extern crate subprocess;

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use clap::{App, Arg, ArgMatches};
use globset::{GlobBuilder, GlobMatcher};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use subprocess::Exec;

fn exec(m: &ArgMatches) {
    let command = m.value_of("COMMAND").unwrap();
    if m.is_present("verbose") {
        println!("{}", command);
    }
    Exec::shell(command).join().expect("Failed");
}

fn visitor(watcher: &mut notify::FsEventWatcher, entry: &DirEntry) {
    match entry.path().to_str() {
        Some(path) => {
            // TODO: Verbose flag
            println!("Matched path: {}", path);
            match watcher.watch(path, RecursiveMode::Recursive) {
                Ok(_) => {}
                Err(e) => {
                    panic!(e);
                }
            }
        }
        None => {
            panic!("Failed to obtain a path to visit!");
        }
    }
}

fn visit_dirs(
    glob: &GlobMatcher,
    watcher: &mut RecommendedWatcher,
    dir: &Path,
    cb: &Fn(&mut RecommendedWatcher, &DirEntry),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(glob, watcher, &path, cb)?;
            } else {
                if glob.is_match(&path) {
                    // TODO: watch
                    cb(watcher, &entry);
                }
            }
        }
    }
    Ok(())
}

fn watch(m: &clap::ArgMatches) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

    if let Some(glob_string) = m.value_of("glob") {
        // TODO: This next line is dangerous as heck
        if let Ok(glob) = GlobBuilder::new(glob_string)
            .literal_separator(false)
            .build()
        {
            let globmatcher = glob.compile_matcher();

            // TODO: Find files that fit this pattern
            visit_dirs(&globmatcher, &mut watcher, Path::new("."), &visitor)?;
        }
    } else if let Some(path) = m.value_of("path") {
        watcher.watch(path, RecursiveMode::Recursive)?;
    }

    loop {
        match rx.recv() {
            Ok(event) => match event {
                notify::DebouncedEvent::Create { .. } => exec(m),
                notify::DebouncedEvent::Write { .. } => exec(m),
                notify::DebouncedEvent::Remove { .. } => exec(m),
                notify::DebouncedEvent::Rename { .. } => exec(m),
                _ => {
                    if m.is_present("verbose") {
                        println!("Ignoring event: {:?}", event);
                    }
                }
            },
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
            Arg::with_name("COMMAND")
                .help("Command ran on response to changes")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Path used for matching files")
                .value_name("FILE/FOLDER")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Enables verbose output")
                .short("v")
                .long("verbose"),
        )
        .arg(
            Arg::with_name("glob")
                .short("g")
                .long("glob")
                .help("Glob used for matching files")
                .value_name("GLOB")
                .takes_value(true),
        )
        .get_matches();

    if let Err(e) = watch(&matches) {
        println!("error: {:?}", e)
    }
}
