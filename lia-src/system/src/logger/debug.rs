use std::{
    fs::OpenOptions,
    io::Write,
    panic::Location,
};
use chrono::Local as time;

use super::{ILogger, LoggerEssentials, Configs};
use crate::{config::get_process_name, path::{join_root, Path, SysPath}};

/// Logger for development purposes. This Logger will save the logs in a .txt file.
pub(super) struct DebugLogger {
    folder: SysPath,
    file_name: String,
}

macro_rules! log_level_impl {
    ($log_level:ident) => {
        #[track_caller]
        fn $log_level<T: AsRef<str>>(message: T, show: bool) {
            let logger: Self = LoggerEssentials::open();
            let save = Configs::get_save();
            let should_log = Configs::get_log().kinds.$log_level;

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

impl ILogger for DebugLogger {
    log_level_impl!(info);
    log_level_impl!(trace);
    log_level_impl!(warn);
    log_level_impl!(error);
}

impl LoggerEssentials for DebugLogger {
    fn open() -> Self {
        let folder = join_root!("logs");

        return DebugLogger {
            folder,
            file_name: format!("{}_log.txt", get_process_name()),
        };
    }

    fn save(&self, message: &String) {
        let path = self.folder.join(&self.file_name);

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path);
        match file {
            Ok(_) => {
                let message = format!("{}\n", message);
                let ok_file = file.as_mut().unwrap();
                ok_file.write_all(message.as_bytes()).unwrap();
            },
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure log is on and save is true (adjust the system/configs.json file)
    #[test]
    fn test_logger() {
        DebugLogger::info("Test info message", true);
        DebugLogger::trace("Test trace message".to_string(), true);
        DebugLogger::warn(&"Test warning message".to_string(), true);
        let test: String = String::from("Test error message");
        DebugLogger::error(test, true);
    }
}
