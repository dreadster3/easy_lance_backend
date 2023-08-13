use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

use crate::{repository::errors::RepositoryError, service::errors::ServiceError};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0} already exists")]
    DuplicateError(String),
    #[error("Dependecy entity with id {0} not found")]
    DependencyError(i32),
    #[error("{0}")]
    RepositoryError(RepositoryError),
    #[error("{0}")]
    ServiceError(ServiceError),
    #[error("Unauthorized")]
    UnauthorizedError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::RepositoryError(err) => err.status_code(),
            ApiError::DependencyError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::DuplicateError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::UnauthorizedError => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::ServiceError(err) => err.status_code(),
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match self.status_code() {
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR => {
                log::error!("{:?}: {}", self, self)
            }
            _ => log::warn!("{:?}: {}", self, self),
        };

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

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        ApiError::ServiceError(error)
    }
}
