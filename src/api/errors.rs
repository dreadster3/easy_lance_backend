use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

use crate::repository::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    RepositoryError(RepositoryError),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::RepositoryError(err) => err.status_code(),
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("{:?}", self);

        return HttpResponse::build(self.status_code()).json(json!({
            "status": self.status_code().as_u16(),
            "error": self.to_string()
        }));
    }
}

impl From<RepositoryError> for ApiError {
    fn from(error: RepositoryError) -> Self {
        ApiError::RepositoryError(error)
    }
}
