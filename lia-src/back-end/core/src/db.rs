use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;
use chrono::Utc;

use crate::models::command::{Command, NewCommand};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .expect("Failed to create pool.");
        Self { pool }
    }

    pub async fn add_command(&self, new_cmd: NewCommand) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        sqlx::query!(
            r#"
            INSERT INTO commands (id, name, description, command_text, tags, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            id,
            new_cmd.name,
            new_cmd.description,
            new_cmd.command_text,
            new_cmd.tags.as_deref(),
            now,
            now,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_all_commands(&self) -> Result<Vec<Command>, sqlx::Error> {
        let rows = sqlx::query_as!(
            Command,
            r#"
            SELECT id, name, description, command_text, tags, created_at, updated_at
            FROM commands
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_command_by_name(&self, cmd_name: &str) -> Result<Command, sqlx::Error> {
        let cmd = sqlx::query_as!(
            Command,
            r#"
            SELECT id, name, description, command_text, tags, created_at, updated_at
            FROM commands
            WHERE name = $1
            "#,
            cmd_name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(cmd)
    }
}
