use crate::entity::job_type::JobType;

use super::errors::RepositoryError;

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<JobType>> {
    let result = match sqlx::query_as!(JobType, "SELECT * FROM tb_job_types")
        .fetch_all(pool)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn get_by_id_async(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<JobType> {
    let result = match sqlx::query_as!(JobType, "SELECT * FROM tb_job_types WHERE id = $1", id)
        .fetch_one(pool)
        .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::NotFound(id)),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn create_async(pool: &sqlx::Pool<sqlx::Postgres>, job_type: JobType) -> Result<JobType> {
    let result = match sqlx::query_as!(
        JobType,
        "INSERT INTO tb_job_types (name, modified_at) VALUES ($1, $2) RETURNING *",
        job_type.name,
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
        "UPDATE tb_job_types SET name = $1, modified_at = $2 WHERE id = $3 RETURNING *",
        job_type.name,
        job_type.modified_at,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}
