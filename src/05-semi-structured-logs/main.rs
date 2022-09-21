#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

impl LogLevel {
    const INFO: &'static str = "INFO";
    const WARNING: &'static str = "WARNING";
    const ERROR: &'static str = "ERROR";

    fn value(&self) -> &'static str {
        match self {
            LogLevel::Info => Self::INFO,
            LogLevel::Warning => Self::WARNING,
            LogLevel::Error => Self::ERROR,
        }
    }
}

fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", LogLevel::value(&level), message)
}
fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

pub fn main() -> () {
    println!("{}", info("This is an INFO message."));
    println!("{}", warn("This is an WARNING message."));
    println!("{}", error("This is an ERROR message."));
}
