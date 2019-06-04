use std::env;
use std::path::PathBuf;
use std::time::Duration;

use std::sync::mpsc::{channel, RecvTimeoutError};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

fn main() {
    let watch_file = env::args()
        .skip(1)
        .next()
        .map(PathBuf::from)
        .expect("please pass a path to the python file we're watching.");
    if !watch_file.exists() || !watch_file.is_file() {
        eprintln!("The path {:?} does not exist or is not a file", watch_file);
        std::process::exit(1);
    }

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).expect("failed to create watcher");
    watcher.watch(watch_file, RecursiveMode::NonRecursive).expect("failed to watch file");

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(DebouncedEvent::Write(p)) => println!("wrote to {:?}", p),
            Ok(event) => println!("{:?}", event),
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
}
