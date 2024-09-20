use crate::errors::LiaCoreError;
use std::process::{Command, Output};
use std::path::Path;

pub struct CmdEngine;

impl CmdEngine {
    pub fn execute_command(command_text: &str, path: &Path) -> Result<Output, LiaCoreError> {
        if !path.exists() || !path.is_dir() {
            return Err(LiaCoreError::InvalidInputError(format!(
                "Path does not exist or is not a directory: {}",
                path.display()
            )));
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(command_text)
            .current_dir(path)
            .output()
            .map_err(LiaCoreError::IoError)?;

        if output.status.success() {
            Ok(output)
        } else {
            Err(LiaCoreError::CommandExecutionError(format!(
                "Command exited with status code {}",
                output.status.code().unwrap_or(-1)
            )))
        }
    }
}
