use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiaCoreError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Command not found: {0}")]
    CommandNotFoundError(String),

    #[error("Failed to start the database container: {0}")]
    DatabaseContainerError(String),

    #[error("Invalid input: {0}")]
    InvalidInputError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Failed to run migrations: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}
