use std::io::BufRead;

use crate::{level::LogLevel, record::LogRecord};

pub fn process_stdin(level_filter: LogLevel) {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(r) => {
                if r.level >= level_filter {
                    print!("{}", r.format())
                }
            }
            Err(e) => {
                println!("{line} {e}")
            }
        }
    }
}
