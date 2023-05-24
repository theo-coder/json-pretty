use std::str::FromStr;

#[derive(clap::ValueEnum, Debug, Clone, serde::Deserialize, PartialEq, PartialOrd, Copy)]
pub enum LogLevel {
    FATAL = 60,
    ERROR = 50,
    WARN = 40,
    INFO = 30,
    DEBUG = 20,
    TRACE = 10,
}

impl FromStr for LogLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fatal" => Ok(LogLevel::FATAL),
            "error" => Ok(LogLevel::ERROR),
            "warn" => Ok(LogLevel::WARN),
            "info" => Ok(LogLevel::INFO),
            "debug" => Ok(LogLevel::DEBUG),
            "trace" => Ok(LogLevel::TRACE),
            _ => Err(anyhow::anyhow!(format!("Invalid level value: '{s}'"))),
        }
    }
}
