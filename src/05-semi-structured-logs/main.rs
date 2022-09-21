#[derive(Clone, PartialEq, Debug)]

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    let level_tag: &str = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    };
    format!("[{}]: {}", level_tag, message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

pub fn main() -> () {
    println!("{}", info("This is an INFO message."));
    println!("{}", warn("This is an WARNING message."));
    println!("{}", error("This is an ERROR message."));
}
