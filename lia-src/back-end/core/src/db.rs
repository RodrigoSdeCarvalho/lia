use sqlx::{
    postgres::PgPoolOptions, 
    PgPool,
};
use uuid::Uuid;
use chrono::Utc;

use crate::models::command::{Command, NewCommand, UpdateCommand};
use crate::errors::LiaCoreError;
use sqlx::Error as SqlxError;

use system::Logger;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn initialize_database(database_url: &str) -> Result<(), LiaCoreError> {
        let status = std::process::Command::new("docker-compose")
            .args(&["up", "-d"])
            .status()
            .map_err(|e| LiaCoreError::DatabaseContainerError(e.to_string()))?;

        if !status.success() {
            return Err(LiaCoreError::DatabaseContainerError(format!(
                "Docker exited with status: {}",
                status
            )));
        }

        Logger::info("Waiting for the database to start...", true);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        Logger::info("Initializing the database...", true);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(LiaCoreError::DatabaseError)?;

        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .map_err(LiaCoreError::MigrationError)?;

        Logger::info("Database initialized successfully.", true);
        Ok(())
    }

    pub async fn new(database_url: &str) -> Result<Self, LiaCoreError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(LiaCoreError::DatabaseError)?;
        Ok(Self { pool })
    }

    pub async fn add_command(&self, new_cmd: NewCommand) -> Result<(), LiaCoreError> {
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
        .await
        .map_err(LiaCoreError::DatabaseError)?;
        Ok(())
    }

    pub async fn update_command(
        &self,
        update_cmd: UpdateCommand,
    ) -> Result<(), LiaCoreError> {
        let mut tx = self.pool.begin().await.map_err(LiaCoreError::DatabaseError)?;
    
        if let Some(tags) = update_cmd.new_tags {
            sqlx::query!(
                r#"
                UPDATE commands
                SET tags = $1, updated_at = $2
                WHERE name = $3  -- Use name instead of id
                "#,
                &tags,
                Utc::now().naive_utc(),
                update_cmd.name
            )
            .execute(&mut *tx)  // Dereference `tx` here
            .await
            .map_err(LiaCoreError::DatabaseError)?;
        }
    
        if let Some(description) = update_cmd.new_description {
            sqlx::query!(
                r#"
                UPDATE commands
                SET description = $1, updated_at = $2
                WHERE name = $3  -- Use name instead of id
                "#,
                description,
                Utc::now().naive_utc(),
                update_cmd.name
            )
            .execute(&mut *tx)  // Dereference `tx` here
            .await
            .map_err(LiaCoreError::DatabaseError)?;
        }
    
        if let Some(command_text) = update_cmd.new_command_text {
            sqlx::query!(
                r#"
                UPDATE commands
                SET command_text = $1, updated_at = $2
                WHERE name = $3  -- Use name instead of id
                "#,
                command_text,
                Utc::now().naive_utc(),
                update_cmd.name
            )
            .execute(&mut *tx)  // Dereference `tx` here
            .await
            .map_err(LiaCoreError::DatabaseError)?;
        }
    
        tx.commit().await.map_err(LiaCoreError::DatabaseError)?;
        Ok(())
    }    

    pub async fn get_all_commands(&self) -> Result<Vec<Command>, LiaCoreError> {
        let rows = sqlx::query_as!(
            Command,
            r#"
            SELECT id, name, description, command_text, tags, created_at, updated_at
            FROM commands
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(LiaCoreError::DatabaseError)?;
        Ok(rows)
    }

    pub async fn get_command_by_name(&self, cmd_name: &str) -> Result<Command, LiaCoreError> {
        let result = sqlx::query_as!(
            Command,
            r#"
            SELECT id, name, description, command_text, tags, created_at, updated_at
            FROM commands
            WHERE name = $1
            "#,
            cmd_name
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(cmd) => Ok(cmd),
            Err(SqlxError::RowNotFound) => Err(LiaCoreError::CommandNotFoundError(cmd_name.to_string())),
            Err(e) => Err(LiaCoreError::DatabaseError(e)),
        }
    }
}
