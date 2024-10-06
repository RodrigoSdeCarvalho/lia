use sqlx::{
    postgres::PgPoolOptions, 
    PgPool,
    Error as SqlxError
};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    models::command::{Command, NewCommand, UpdateCommand},
    errors::LiaCoreError
};

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
            .execute(&mut *tx)
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
            .execute(&mut *tx)
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
            .execute(&mut *tx)
            .await
            .map_err(LiaCoreError::DatabaseError)?;
        }
    
        tx.commit().await.map_err(LiaCoreError::DatabaseError)?;
        Ok(())
    }    

    pub async fn get_all_commands(&self, limit: i64, offset: i64) -> Result<Vec<Command>, LiaCoreError> {
        let rows = sqlx::query_as!(
            Command,
            r#"
            SELECT id, name, description, command_text, tags, created_at, updated_at
            FROM commands
            ORDER BY name
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
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

    pub async fn search_commands(
        &self,
        query: &str,
        tags: Option<Vec<String>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Command>, LiaCoreError> {
        let search_query = if !query.is_empty() {
            Some(format!("{}:*", query.replace(" ", " & ")))
        } else {
            None
        };
    
        let commands = match (search_query, tags) {
            (Some(sq), Some(tags_vec)) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    WHERE
                        search_vector @@ to_tsquery('english', $1)
                        AND tags && $2::text[]
                    ORDER BY ts_rank(search_vector, to_tsquery('english', $1)) DESC
                    LIMIT $3 OFFSET $4
                    "#,
                    sq,
                    &tags_vec,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?
            }
            (Some(sq), None) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    WHERE
                        search_vector @@ to_tsquery('english', $1)
                    ORDER BY ts_rank(search_vector, to_tsquery('english', $1)) DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    sq,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?
            }
            (None, Some(tags_vec)) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    WHERE
                        tags && $1::text[]
                    ORDER BY name
                    LIMIT $2 OFFSET $3
                    "#,
                    &tags_vec,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?
            }
            (None, None) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    ORDER BY name
                    LIMIT $1 OFFSET $2
                    "#,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?
            }
        };
        Ok(commands)
    }    

    pub async fn find_commands_for_deletion(
        &self,
        name: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<Vec<Command>, LiaCoreError> {
        match (name, tags) {
            (Some(name), Some(tags_vec)) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    WHERE
                        name = $1
                        AND tags && $2::text[]
                    "#,
                    name,
                    &tags_vec,
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)
            }
            (Some(name), None) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    WHERE
                        name = $1
                    "#,
                    name,
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)
            }
            (None, Some(tags_vec)) => {
                sqlx::query_as!(
                    Command,
                    r#"
                    SELECT id, name, description, command_text, tags, created_at, updated_at
                    FROM commands
                    WHERE
                        tags && $1::text[]
                    "#,
                    &tags_vec,
                )
                .fetch_all(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)
            }
            (None, None) => {
                Ok(vec![])
            }
        }
    }

    pub async fn delete_commands(
        &self,
        name: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<(), LiaCoreError> {
        match (name, tags) {
            (Some(name), Some(tags_vec)) => {
                sqlx::query!(
                    r#"
                    DELETE FROM commands
                    WHERE
                        name = $1
                        AND tags && $2::text[]
                    "#,
                    name,
                    &tags_vec,
                )
                .execute(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?;
            }
            (Some(name), None) => {
                sqlx::query!(
                    r#"
                    DELETE FROM commands
                    WHERE
                        name = $1
                    "#,
                    name,
                )
                .execute(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?;
            }
            (None, Some(tags_vec)) => {
                sqlx::query!(
                    r#"
                    DELETE FROM commands
                    WHERE
                        tags && $1::text[]
                    "#,
                    &tags_vec,
                )
                .execute(&self.pool)
                .await
                .map_err(LiaCoreError::DatabaseError)?;
            }
            (None, None) => {}
        }
        Ok(())
    }

    pub async fn delete_all_commands(&self) -> Result<(), LiaCoreError> {
        sqlx::query!("DELETE FROM commands")
            .execute(&self.pool)
            .await
            .map_err(LiaCoreError::DatabaseError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Executor, PgPool};
    use system::EnvConfig;

    async fn setup_test_db() -> PgPool {
        let database_url = EnvConfig::get_database_url();

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS commands (
                id UUID PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,  // Ensure name is unique
                description TEXT,
                command_text TEXT,
                tags TEXT[],
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
            "#
        )
        .await
        .expect("Failed to create commands table");

        pool
    }

    async fn teardown_test_db(pool: &PgPool) {
        pool.execute("DROP TABLE IF EXISTS commands")
            .await
            .expect("Failed to drop commands table");
    }

    #[tokio::test]
    async fn test_add_command() {
        let pool = setup_test_db().await;
        let db = Database { pool: pool.clone() };

        let new_command = NewCommand {
            name: String::from("Test Command Add"),
            description: Some(String::from("Test Description")),
            command_text: String::from("echo Hello, World!"),
            tags: Some(vec!["test".to_string(), "command".to_string()]),
        };

        let result = db.add_command(new_command).await;
        assert!(result.is_ok());

        let commands = db.get_all_commands(10, 0).await.expect("Failed to fetch commands");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "Test Command Add");

        teardown_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_update_command() {
        let pool = setup_test_db().await;
        let db = Database { pool: pool.clone() };

        let new_command = NewCommand {
            name: String::from("Test Command Update"),
            description: Some(String::from("Test Description")),
            command_text: String::from("echo Hello, World!"),
            tags: Some(vec!["test".to_string(), "command".to_string()]),
        };

        db.add_command(new_command).await.expect("Failed to add command");

        let update_command = UpdateCommand {
            name: String::from("Test Command Update"),
            new_description: Some(String::from("Updated Description")),
            new_command_text: None,
            new_tags: None,
        };

        let result = db.update_command(update_command).await;
        assert!(result.is_ok());

        let command = db.get_command_by_name("Test Command Update").await.expect("Failed to fetch command");
        assert_eq!(command.description.unwrap(), "Updated Description");

        teardown_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_delete_command() {
        let pool = setup_test_db().await;
        let db = Database { pool: pool.clone() };

        let new_command = NewCommand {
            name: String::from("Test Command Delete"),
            description: Some(String::from("Test Description")),
            command_text: String::from("echo Hello, World!"),
            tags: Some(vec!["test".to_string(), "command".to_string()]),
        };

        db.add_command(new_command).await.expect("Failed to add command");

        db.delete_commands(Some("Test Command Delete".to_string()), None)
            .await
            .expect("Failed to delete command");

        let commands = db.get_all_commands(10, 0).await.expect("Failed to fetch commands");
        assert_eq!(commands.len(), 0);

        teardown_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_search_commands() {
        let pool = setup_test_db().await;
        let db = Database { pool: pool.clone() };

        let new_command1 = NewCommand {
            name: String::from("Test Command Search 1"),
            description: Some(String::from("First test command")),
            command_text: String::from("echo Test 1"),
            tags: Some(vec!["test".to_string(), "command".to_string()]),
        };

        let new_command2 = NewCommand {
            name: String::from("Test Command Search 2"),
            description: Some(String::from("Second test command")),
            command_text: String::from("echo Test 2"),
            tags: Some(vec!["test".to_string(), "search".to_string()]),
        };

        db.add_command(new_command1).await.expect("Failed to add command 1");
        db.add_command(new_command2).await.expect("Failed to add command 2");

        let search_results = db
            .search_commands("Test", Some(vec!["command".to_string()]), 10, 0)
            .await
            .expect("Failed to search commands");

        assert_eq!(search_results.len(), 2);

        teardown_test_db(&pool).await;
    }
}
