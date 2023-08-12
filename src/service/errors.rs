use thiserror::Error;

use crate::repository::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum ServiceError {
    RepositoryError(RepositoryError),
}
