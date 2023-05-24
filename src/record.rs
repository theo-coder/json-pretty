use itertools::Itertools;
use serde::Serialize;
use std::borrow::Cow;

use colored::Colorize;
use serde_json::{ser::PrettyFormatter, Serializer};

use crate::level::LogLevel;

#[derive(serde::Deserialize)]
pub struct LogRecord<'a> {
    pub time: String,
    pub level: LogLevel,
    pub message: Cow<'a, str>,
    #[serde(flatten)]
    pub extras: serde_json::Map<String, serde_json::Value>,
}

impl<'a> LogRecord<'a> {
    pub fn format(&self) -> String {
        let level = format_level(self.level);
        let formatted = format!(
            "[{}] {}: {}{}",
            self.time,
            level,
            self.message.bright_white().on_black(),
            format_extras(&self.extras)
        );
        formatted
    }
}

pub fn format_level(level: LogLevel) -> String {
    match level {
        LogLevel::TRACE => "TRACE".white(),
        LogLevel::WARN => "WARN".magenta(),
        LogLevel::INFO => "INFO".cyan(),
        LogLevel::FATAL => "FATAL".reversed(),
        LogLevel::ERROR => "ERROR".red(),
        LogLevel::DEBUG => "DEBUG".yellow(),
    }
    .to_string()
}

pub fn format_extras(extra_fields: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut details = Vec::new();
    let mut extras = Vec::new();
    for (key, value) in extra_fields {
        if ["line", "target", "file", "pid", "name", "host"].contains(&key.as_str()) {
            continue;
        }
        let stringified = if let serde_json::Value::String(s) = value {
            // Preserve strings unless they contain whitespaces/are empty
            // In that case, we want surrounding quotes.
            if s.contains(' ') || s.is_empty() {
                format!("\"{}\"", s)
            } else {
                s.to_owned()
            }
        } else {
            json_to_indented_string(value, "  ")
        };

        if stringified.contains('\n') || stringified.len() > 50 {
            if let serde_json::Value::String(s) = value {
                details.push(indent(&format!("{}: {}", key.bold(), s)));
            } else {
                details.push(indent(&format!("{}: {}", key.bold(), stringified)));
            }
        } else {
            extras.push(format!("{}={}", key.bold(), stringified));
        }
    }
    let formatted_details = if !details.is_empty() {
        format!("{}\n", details.into_iter().join("\n    --\n"))
    } else {
        "".into()
    };
    let formatted_extras = if !extras.is_empty() {
        format!(" ({})", extras.into_iter().join(","))
    } else {
        "".into()
    };
    format!("{}\n{}", formatted_extras, formatted_details)
}

fn json_to_indented_string(value: &serde_json::Value, indent: &str) -> String {
    let mut writer = Vec::with_capacity(128);
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());
    let mut serializer = Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer).unwrap();
    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

pub fn indent(s: &str) -> String {
    format!("    {}", s.lines().join("\n    "))
}
