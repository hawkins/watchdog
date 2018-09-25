extern crate notify;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch(path: &str) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

    try!(watcher.watch(path, RecursiveMode::Recursive));

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    if let Err(e) = watch(&args[1]) {
        println!("error: {:?}", e)
    }
}
