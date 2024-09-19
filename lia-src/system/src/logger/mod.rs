mod debug;

use std::panic::Location;
use chrono::Local as time;

use super::config::{Configs, Profile, get_process_name};
use debug::DebugLogger;

macro_rules! log {
    ($log_level:ident) => {
        pub fn $log_level<T: AsRef<str>>(message: T, show: bool) {
            let profile = {
                let config = Configs::open().lock().unwrap();
                config.profile().clone()
            };

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
        fn $log_level<T: AsRef<str>>(message: T, show: bool) {
            let logger: Self = LoggerEssentials::open();

            let (should_log, save): (bool, bool) = {
                let config = Configs::open().lock().unwrap();

                let should_log = config.log().on && config.log().kinds.$log_level;
                let save = config.save();

                (should_log, save)
            };

            if should_log {
                let location = Location::caller();
                let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let message = format!(
                    "{}::[{:?}] {} [{}:{}] - {}",
                    get_process_name(),
                    stringify!($log_level).to_uppercase(),
                    timestamp,
                    location.file(),
                    location.line(),
                    message.as_ref()
                );

                if save { logger.save(&message); }

                if show {
                    println!("{}", message); 
                }
            }
        }
    };
}

trait ILogger where Self: LoggerEssentials {
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
