use thiserror::Error;

/// All possible errors for the todo CLI application.
#[derive(Error, Debug)]
pub enum TodoError {
    /// IO error (file system, etc).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON serialization/deserialization error.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    /// Date parsing error.
    #[error("Date parsing error: {0}")]
    DateParse(String),
    /// Task not found by ID.
    #[error("Task not found with ID: {0}")]
    TaskNotFound(u64),
    /// Invalid priority value (must be 1-5).
    #[error("Invalid priority value: {0}. Priority must be between 1 and 5")]
    InvalidPriority(u8),
    /// Invalid tag (empty or malformed).
    #[error("Invalid tag: {0}. Tags cannot be empty")]
    InvalidTag(String),
    /// Data file corruption or unreadable.
    #[error("Data file corruption: {0}")]
    DataCorruption(String),
}

/// Result type for all todo CLI operations.
pub type Result<T> = std::result::Result<T, TodoError>;
