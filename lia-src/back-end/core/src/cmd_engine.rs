use crate::errors::LiaCoreError;
use std::{
    io::{BufRead, BufReader}, 
    path::Path, 
    process::{Command, Output, Stdio}
};

#[cfg(target_os = "linux")]
use libc;

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

    pub fn execute_command_stream(
        command_text: &str,
        path: &Path,
        output_tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<(), LiaCoreError> {
        if !path.exists() || !path.is_dir() {
            return Err(LiaCoreError::InvalidInputError(format!(
                "Path does not exist or is not a directory: {}",
                path.display()
            )));
        }

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command_text)
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(LiaCoreError::IoError)?;

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                output_tx.send(line).unwrap_or_else(|_| ());
            }
        }

        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                output_tx.send(format!("ERROR: {}", line)).unwrap_or_else(|_| ());
            }
        }

        let status = child.wait().map_err(LiaCoreError::IoError)?;

        if status.success() {
            Ok(())
        } else {
            Err(LiaCoreError::CommandExecutionError(format!(
                "Command exited with status code {}",
                status.code().unwrap_or(-1)
            )))
        }
    }


    #[cfg(target_os = "linux")]
    pub fn is_sudo_user() -> bool {
        let euid = unsafe { libc::geteuid() };
        if euid == 0 {
            true
        } else {
            false
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_sudo_user() -> bool {
        false
    }
}
