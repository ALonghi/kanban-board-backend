use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

/// Our app's top level error type.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("action in tasks repo failed")]
    TaskRepo(TaskRepoError),
    #[error("action in boards repo failed")]
    BoardRepo(BoardRepoError),
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
}

/// Errors that can happen when using the task repo.
#[derive(Error, Debug)]
pub enum TaskRepoError {
    #[allow(dead_code)]
    #[error("task not found")]
    NotFound,
    #[allow(dead_code)]
    #[error("task is invalid: {0}")]
    InvalidTask(String),
    #[allow(dead_code)]
    #[error("decoding task resulted in an error: {0}")]
    DecodeError(String),
}

/// Errors that can happen when using the task repo.
#[derive(Error, Debug)]
pub enum BoardRepoError {
    #[allow(dead_code)]
    #[error("board not found")]
    NotFound,
    #[allow(dead_code)]
    #[error("board is invalid: {0}")]
    InvalidBoard(String),
    #[allow(dead_code)]
    #[error("decoding board resulted in an error: {0}")]
    DecodeError(String),
}

/// This makes it possible to use `?` to automatically convert a `TaskRepoError`
/// into an `AppError`.
impl From<TaskRepoError> for AppError {
    fn from(inner: TaskRepoError) -> Self {
        AppError::TaskRepo(inner)
    }
}

/// This makes it possible to use `?` to automatically convert a `TaskRepoError`
/// into an `AppError`.
impl From<BoardRepoError> for AppError {
    fn from(inner: BoardRepoError) -> Self {
        AppError::BoardRepo(inner)
    }
}
