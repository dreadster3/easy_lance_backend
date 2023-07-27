use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Object with id {0} not found")]
    NotFound(i32),
    #[error("Database Error")]
    InternalError(sqlx::Error),
}

impl ResponseError for RepositoryError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            RepositoryError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            RepositoryError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
