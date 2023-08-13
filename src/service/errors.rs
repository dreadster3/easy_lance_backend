use actix_web::ResponseError;
use thiserror::Error;

use crate::repository::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{0}")]
    RepositoryError(RepositoryError),
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        return Self::RepositoryError(err);
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::RepositoryError(err) => err.status_code(),
        }
    }
}
