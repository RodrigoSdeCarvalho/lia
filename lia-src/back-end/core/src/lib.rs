pub mod models;
mod db;
pub mod errors;
mod cmd_engine;

use std::{
    path::Path,
    process::Output, sync::mpsc::Sender
};

use system::{Logger, Config};

use db::Database;
use errors::LiaCoreError;
use models::command::{Command, NewCommand};

pub struct LiaCore {
    pub db: Database,
}

impl LiaCore {
    pub async fn init() -> Result<(), LiaCoreError> {
        Logger::info("Initializing the database.", true);
        let database_url = Config::get_database_url();

        let res = Database::initialize_database(&database_url).await;
        res
    }

    pub async fn new() -> Result<Self, LiaCoreError> {
        let database_url = Config::get_database_url();
    
        let db = Database::new(&database_url).await;
        match db {
            Ok(pool) => {
                Logger::info("Database connection established.", true);
                Ok(Self { db: pool })
            },
            Err(e) => {
                Logger::error(&format!("Failed to connect to database: {}", e), true);
                Err(e)
            },
        }
    }

    pub async fn add_command(&self, new_cmd: NewCommand) -> Result<(), LiaCoreError> {
        Logger::info(&format!("Adding command: {}", new_cmd.name), true);
        let comm = self.db.add_command(new_cmd).await;
        match comm {
            Ok(_) => Ok(Logger::info("Command added successfully.", true)),
            Err(e) => {
                Logger::error(&format!("Failed to add command: {}", e), true);
                Err(e)
            },
        }
    }

    pub async fn get_all_commands(&self) -> Result<Vec<Command>, LiaCoreError> {
        Logger::info("Getting all commands.", true);
        let comms = self.db.get_all_commands().await;
        match comms {
            Ok(commands) => {
                Logger::info("Commands retrieved successfully. \n", true);
                Ok(commands)
            },
            Err(e) => {
                Logger::error(&format!("Failed to get commands: {}", e), true);
                Err(e)
            },
        }
    }

    pub async fn get_command_by_name(&self, cmd_name: &str) -> Result<Command, LiaCoreError> {
        Logger::info(&format!("Getting command by name: {}", cmd_name), true);
        let comm = self.db.get_command_by_name(cmd_name).await;
        match comm {
            Ok(command) => {
                Logger::info("Command retrieved successfully.", true);
                Ok(command)
            },
            Err(e) => {
                Logger::error(&format!("Failed to get command: {}", e), true);
                Err(e)
            },
        }
    }

    pub async fn run_command(&self, cmd: Command, path: &Path) -> Result<Output, LiaCoreError> {
        cmd_engine::CmdEngine::execute_command(&cmd.command_text, path)
    }

    pub async fn run_command_stream(
        &self,
        cmd: Command,
        path: &Path,
        output_tx: Sender<String>,
    ) -> Result<(), LiaCoreError> {
        cmd_engine::CmdEngine::execute_command_stream(&cmd.command_text, path, output_tx)
    }
}
