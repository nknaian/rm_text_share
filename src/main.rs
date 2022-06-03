use syslog::{Facility, Formatter3164, BasicLogger};
use std::thread::sleep;

fn main() {
    // Setup logging to syslog (at info level)
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "rm_text_share".into(),
        pid: 0,
    };
    
    let logger = match syslog::unix(formatter) {
        Err(e) => { println!("impossible to connect to syslog: {:?}", e); return; },
        Ok(logger) => logger,
    };
    let _ = log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(log::LevelFilter::Info));

    log::info!("Started rm_text share");

    // Main loop
    loop {
        sleep(std::time::Duration::from_secs(10));
        log::info!("Another 10 seconds have elapsed, hurrah!");
    }
}
