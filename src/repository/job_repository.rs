use crate::entity::job::Job;

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn get_all_async(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Job>> {
    let result = match sqlx::query_as!(Job, "SELECT * FROM tb_jobs")
        .fetch_all(pool)
        .await
    {
        Ok(jobs) => Ok(jobs),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn get_by_id_async(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Job> {
    let result = match sqlx::query_as!(Job, "SELECT * FROM tb_jobs WHERE id = $1", id)
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
    let result = match sqlx::query_as!(Job,
        "INSERT INTO tb_jobs (name, description, start_date, end_date, job_type_id, modified_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        job.name,
        job.description,
        job.start_date,
        job.end_date,
        job.job_type_id,
        job.modified_at
    )
    .fetch_one(pool)
    .await {
        Ok(job) => Ok(job),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}

pub async fn update_async(pool: &sqlx::Pool<sqlx::Postgres>, id: i32, job: Job) -> Result<Job> {
    let result = match sqlx::query_as!(Job,
        "UPDATE tb_jobs SET name = $1, description = $2, start_date = $3, end_date = $4, job_type_id = $5, modified_at = $6 WHERE id = $7 RETURNING *",
        job.name,
        job.description,
        job.start_date,
        job.end_date,
        job.job_type_id,
        job.modified_at,
        id
    )
    .fetch_one(pool)
    .await {
        Ok(job) => Ok(job),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(err) => Err(RepositoryError::InternalError(err)),
    };

    return result;
}
