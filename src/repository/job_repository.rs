use crate::entity::job::Job;

pub async fn get_all_async(pool: &sqlx::Pool<sqlx::Postgres>) -> Vec<Job> {
    let jobs = sqlx::query_as!(Job, "SELECT * FROM tb_jobs")
        .fetch_all(pool)
        .await
        .unwrap();

    return jobs;
}

pub async fn create_async(pool: &sqlx::Pool<sqlx::Postgres>, job: Job) -> Job {
    let job_inserted = sqlx::query_as!(Job,
        "INSERT INTO tb_jobs (name, description, start_date, end_date, job_type_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        job.name,
        job.description,
        job.start_date,
        job.end_date,
        job.job_type_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    return job_inserted;
}
