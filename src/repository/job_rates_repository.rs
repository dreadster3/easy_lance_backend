use crate::entity::job_rate::JobRate;

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobRate>> {
    let result = match sqlx::query_as!(
        JobRate,
        "SELECT * FROM tb_job_rates WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(jobs) => Ok(jobs),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn get_by_id_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<JobRate> {
    let result = match sqlx::query_as!(
        JobRate,
        "SELECT * FROM tb_job_rates WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn create_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    job_rate: JobRate,
) -> Result<JobRate> {
    let result = match sqlx::query_as!(
        JobRate,
        "INSERT INTO tb_job_rates (rate, threshold, user_id) VALUES ($1, $2, $3) RETURNING *",
        job_rate.rate,
        job_rate.threshold,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn update_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
    job_rate: JobRate,
) -> Result<JobRate> {
    let result = match sqlx::query_as!(
        JobRate,
        "UPDATE tb_job_rates SET rate = $1, threshold = $2, modified_at = $3 WHERE id = $4 AND user_id = $5 RETURNING *",
        job_rate.rate,
        job_rate.threshold,
        job_rate.modified_at,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(job_rate.id))),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn delete_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<JobRate> {
    let result = match sqlx::query_as!(
        JobRate,
        "DELETE FROM tb_job_rates WHERE id = $1 AND user_id = $2 RETURNING *",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}
