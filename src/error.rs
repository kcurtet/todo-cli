use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Date parsing error: {0}")]
    DateParse(String),

    #[error("Task not found with ID: {0}")]
    TaskNotFound(u64),

    #[error("Invalid priority value: {0}. Priority must be between 1 and 5")]
    InvalidPriority(u8),

    #[error("Invalid tag: {0}. Tags cannot be empty")]
    InvalidTag(String),

    #[error("Data file corruption: {0}")]
    DataCorruption(String),
}

pub type Result<T> = std::result::Result<T, TodoError>;
