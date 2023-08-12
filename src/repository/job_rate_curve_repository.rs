use std::future::{self, Future};

use crate::entity::{job_rate::JobRate, job_rate_curve::JobRateCurve};

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobRateCurve>> {
    let result = match sqlx::query_as("SELECT * FROM tb_job_rate_curves WHERE user_id = $1")
        .bind(user_id)
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
    let result =
        match sqlx::query_as("SELECT * FROM tb_job_rate_curves WHERE user_id = $1 AND id = $2")
            .bind(user_id)
            .bind(id)
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
    let result = match sqlx::query_as(
        "INSERT INTO tb_job_rate_curves (user_id, name) VALUES ($1, $2) RETURNING *",
    )
    .bind(user_id)
    .bind(job_rate_curve.name)
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
    let result = match sqlx::query_as(
        "UPDATE tb_job_rate_curves SET name = $3 WHERE user_id = $1 AND id = $2 RETURNING *",
    )
    .bind(user_id)
    .bind(id)
    .bind(job_rate_curve.name)
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
    let result = match sqlx::query_as(
        "DELETE FROM tb_job_rate_curves WHERE user_id = $1 AND id = $2 RETURNING *",
    )
    .bind(user_id)
    .bind(id)
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

pub async fn check_exists_by_id_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    id: i32,
) -> Result<bool> {
    let result = match sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM tb_job_rate_curves WHERE user_id = $1 AND id = $2)",
        user_id,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => Ok(row.exists.unwrap()),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn check_duplicate_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    job_rate_curve: &JobRateCurve,
) -> Result<bool> {
    let result = match sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM tb_job_rate_curves WHERE user_id = $1 AND name = $2)",
        user_id,
        job_rate_curve.name,
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => Ok(row.exists.unwrap()),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn get_all_with_job_rates_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: i32,
    user_id: i32,
) -> Result<JobRateCurve> {
    let job_rate_curve_promise =
        sqlx::query_as("SELECT * FROM tb_job_rate_curves WHERE user_id = $1 AND id = $2")
            .bind(user_id)
            .bind(id)
            .fetch_one(pool);

    let job_rates_promise =
        sqlx::query_as("SELECT * FROM tb_job_rates WHERE user_id = $1 AND job_rate_curve_id = $2")
            .bind(user_id)
            .bind(id)
            .fetch_all(pool);

    let (job_rate_curve_result, job_rates_result) =
        tokio::join!(job_rate_curve_promise, job_rates_promise);

    let result = match (job_rate_curve_result, job_rates_result) {
        (Ok(job_rate_curve), Ok(job_rates)) => {
            let mut job_rate_curve: JobRateCurve = job_rate_curve;
            job_rate_curve.job_rates = job_rates;
            Ok(job_rate_curve)
        }
        (Err(err), _) => Err(RepositoryError::InternalError(err)),
        (_, Err(err)) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}
