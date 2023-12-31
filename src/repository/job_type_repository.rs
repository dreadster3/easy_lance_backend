use crate::entity::job_type::JobType;

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobType>> {
    let result = match sqlx::query_as!(
        JobType,
        "SELECT * FROM tb_job_types WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn get_by_id_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<JobType> {
    let result = match sqlx::query_as!(
        JobType,
        "SELECT * FROM tb_job_types WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn create_async(pool: &sqlx::Pool<sqlx::Postgres>, job_type: JobType) -> Result<JobType> {
    let result = match sqlx::query_as!(
        JobType,
        "INSERT INTO tb_job_types (name, user_id, modified_at) VALUES ($1, $2, $3) RETURNING *",
        job_type.name,
        job_type.user_id,
        job_type.modified_at
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn update_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: i32,
    job_type: JobType,
) -> Result<JobType> {
    let result = match sqlx::query_as!(
        JobType,
        "UPDATE tb_job_types SET name = $1, modified_at = $2 WHERE id = $3 AND user_id = $4 RETURNING *",
        job_type.name,
        job_type.modified_at,
        id,
        job_type.user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn delete_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<JobType> {
    let result = match sqlx::query_as!(
        JobType,
        "DELETE FROM tb_job_types WHERE id = $1 AND user_id = $2 RETURNING *",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn check_exists_by_name_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    name: &str,
) -> Result<bool> {
    let result = match sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM tb_job_types WHERE name = $1 AND user_id = $2)",
        name,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result.exists.unwrap()),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn check_exists_by_id_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<bool> {
    let result = match sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM tb_job_types WHERE id = $1 AND user_id = $2)",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => Ok(row.exists.unwrap()),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}
