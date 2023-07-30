use crate::entity::job_rate_curve::JobRateCurve;

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobRateCurve>> {
    let result = match sqlx::query_as!(
        JobRateCurve,
        "SELECT * FROM tb_job_rate_curves WHERE user_id = $1",
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
) -> Result<JobRateCurve> {
    let result = match sqlx::query_as!(
        JobRateCurve,
        "SELECT * FROM tb_job_rate_curves WHERE user_id = $1 AND id = $2",
        user_id,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(RepositoryError::from(NotFoundError::ById(id))),
            _ => Err(RepositoryError::InternalError(err)),
        },
    };

    return result;
}

pub async fn create_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    job_rate_curve: JobRateCurve,
) -> Result<JobRateCurve> {
    let result = match sqlx::query_as!(
        JobRateCurve,
        "INSERT INTO tb_job_rate_curves (user_id, name) VALUES ($1, $2) RETURNING *",
        user_id,
        job_rate_curve.name,
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
    job_rate_curve: JobRateCurve,
) -> Result<JobRateCurve> {
    let result = match sqlx::query_as!(
        JobRateCurve,
        "UPDATE tb_job_rate_curves SET name = $3 WHERE user_id = $1 AND id = $2 RETURNING *",
        user_id,
        id,
        job_rate_curve.name,
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(RepositoryError::from(NotFoundError::ById(
                job_rate_curve.id,
            ))),
            _ => Err(RepositoryError::InternalError(err)),
        },
    };

    return result;
}

pub async fn delete_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<JobRateCurve> {
    let result = match sqlx::query_as!(
        JobRateCurve,
        "DELETE FROM tb_job_rate_curves WHERE user_id = $1 AND id = $2 RETURNING *",
        user_id,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(job) => Ok(job),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(RepositoryError::from(NotFoundError::ById(id))),
            _ => Err(RepositoryError::InternalError(err)),
        },
    };

    return result;
}
