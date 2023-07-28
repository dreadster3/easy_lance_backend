use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("{0}")]
    NotFoundError(NotFoundError),

    #[error("Database Error")]
    InternalError(sqlx::Error),
}

#[derive(Error, Debug)]
pub enum NotFoundError {
    #[error("Object with id {0} not found")]
    ById(i32),
    #[error("Object with {0} {1} not found")]
    ByProperty(String, String),
}

impl From<NotFoundError> for RepositoryError {
    fn from(error: NotFoundError) -> Self {
        RepositoryError::NotFoundError(error)
    }
}

impl ResponseError for RepositoryError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            RepositoryError::NotFoundError(_) => actix_web::http::StatusCode::NOT_FOUND,
            RepositoryError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
