mod text_conversion;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use syslog::{BasicLogger, Facility, Formatter3164};

use crate::text_conversion::TextConversion;

fn main() {
    // Setup logging to syslog (at info level)
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "rm_text_share".into(),
        pid: 0,
    };

    let logger = match syslog::unix(formatter) {
        Err(e) => {
            println!("impossible to connect to syslog: {:?}", e);
            return;
        }
        Ok(logger) => logger,
    };
    let _ = log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info));

    log::info!("Started rm_text share");

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, std::time::Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            "/home/root/.local/share/remarkable/xochitl",
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => handle_event(event), // NOTE: How is println working to show up in syslog??
            Err(e) => println!("watch error: {:?}", e),
        }
        // TODO: Need to also check the files in the local folder for updating dropbox...
        // I don't think I can just immediately send updates to dropbox when the events occur, because it's possible that
        // there won't be internet connection...
    }
}

fn handle_event(event: DebouncedEvent) {
    match event {
        DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
            if let Some(text_conversion) = TextConversion::new(path) {
                log::info!("Here's the text conversion: {:?}", text_conversion);
            }
        }
        _ => (),
    }
}
