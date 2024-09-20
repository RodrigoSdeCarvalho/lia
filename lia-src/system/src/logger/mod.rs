mod debug;

use super::config::{Configs, Profile};
use debug::DebugLogger;

macro_rules! log {
    ($log_level:ident) => {
        #[track_caller]
        pub fn $log_level<T: AsRef<str>>(message: T, show: bool) {
            let profile = Configs::get_profile();

            match profile {
                Profile::DEBUG => DebugLogger::$log_level(message, show),
            };
        }
    };
}

pub struct Logger;

impl Logger {
    log!(info);
    log!(trace);
    log!(warn);
    log!(error);
}

macro_rules! log_level {
    ($log_level:ident) => {
        #[track_caller]
        fn $log_level<T: AsRef<str>>(message: T, show: bool);
    };
}

trait ILogger: LoggerEssentials {
    log_level!(info);
    log_level!(trace);
    log_level!(warn);
    log_level!(error);
}


trait LoggerEssentials where Self: Sized {
    fn open() -> Self;
    fn save(&self, message: &String);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::set_process_name;

    // Make sure log is on and save is true (adjust the system/configs.json file)
    #[test]
    fn test_logger() {
        set_process_name("Test");
        Logger::info("Test info message", true);
        Logger::trace("Test trace message".to_string(), true);
        Logger::warn(&"Test warning message".to_string(), true);
        let test: String = String::from("Test error message");
        Logger::error(test, true);
    }
}
