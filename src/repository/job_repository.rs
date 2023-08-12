use crate::entity::job::Job;

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32) -> Result<Vec<Job>> {
    let result = match sqlx::query_as("SELECT * FROM tb_jobs WHERE user_id = $1")
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
) -> Result<Job> {
    let result = match sqlx::query_as("SELECT * FROM tb_jobs WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    {
        Ok(job) => Ok(job),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn create_async(pool: &sqlx::Pool<sqlx::Postgres>, job: Job) -> Result<Job> {
    let result = match sqlx::query_as(
        "INSERT INTO tb_jobs (name, description, start_date, end_date, user_id, job_rate_curve_id, job_type_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(job.name)
    .bind(job.description)
    .bind(job.start_date)
    .bind(job.end_date)
    .bind(job.user_id)
    .bind(job.job_rate_curve_id)
    .bind(job.job_type_id)
    .fetch_one(pool)
    .await {
        Ok(job) => Ok(job),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn update_async(pool: &sqlx::Pool<sqlx::Postgres>, id: i32, job: Job) -> Result<Job> {
    let result = match sqlx::query_as(
        "UPDATE tb_jobs SET name = $1, description = $2, start_date = $3, end_date = $4, job_rate_curve_id = $5, job_type_id = $6, modified_at = $7 WHERE id = $8 AND user_id = $9 RETURNING *",
    )
    .bind(job.name)
    .bind(job.description)
    .bind(job.start_date)
    .bind(job.end_date)
    .bind(job.job_rate_curve_id)
    .bind(job.job_type_id)
    .bind(job.modified_at)
    .bind(id)
    .bind(job.user_id)
    .fetch_one(pool)
    .await {
        Ok(job) => Ok(job),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn delete_async(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32, id: i32) -> Result<Job> {
    let result =
        match sqlx::query_as("DELETE FROM tb_jobs WHERE id = $1 AND user_id = $2 RETURNING *")
            .bind(id)
            .bind(user_id)
            .fetch_one(pool)
            .await
        {
            Ok(job) => Ok(job),
            Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
            Err(err) => Err(RepositoryError::InternalError(err)),
        };

    return result;
}
