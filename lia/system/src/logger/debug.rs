use std::fs::OpenOptions;
use std::io::Write;

use super::{ILogger, LoggerEssentials};
use crate::{config::get_process_name, path::{join_root, Path, SysPath}};

/// Logger for development purposes. This Logger will save the logs in a .txt file.
pub(super) struct DebugLogger {
    folder: SysPath,
    file_name: String,
}

impl ILogger for DebugLogger {}

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
            .open(&path)
            .unwrap();

        let message = format!("{}\n", message);
        file.write_all(message.as_bytes()).unwrap();
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
