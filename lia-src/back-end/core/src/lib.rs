pub mod models;
pub mod db;

use db::Database;
use models::command::{Command, NewCommand};

pub struct LiaCore {
    pub db: Database,
}

impl LiaCore {
    pub async fn new(database_url: &str) -> Self {
        let db = Database::new(database_url).await;
        Self { db }
    }

    pub async fn add_command(&self, new_cmd: NewCommand) -> Result<(), sqlx::Error> {
        self.db.add_command(new_cmd).await
    }

    pub async fn get_all_commands(&self) -> Result<Vec<Command>, sqlx::Error> {
        self.db.get_all_commands().await
    }

    pub async fn get_command_by_name(&self, cmd_name: &str) -> Result<Command, sqlx::Error> {
        self.db.get_command_by_name(cmd_name).await
    }
}
