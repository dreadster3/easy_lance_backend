use crate::entity::job::Job;

pub async fn get_all_async(pool: &sqlx::Pool<sqlx::Postgres>) -> Vec<Job> {
    let jobs = sqlx::query_as::<_, Job>("SELECT * FROM tb_jobs")
        .fetch_all(pool)
        .await
        .unwrap();

    return jobs;
}
