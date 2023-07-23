use crate::entity::job::Job;

pub fn get_all(pool: &sqlx::Pool<sqlx::Postgres>) -> Vec<Job> {
    return vec![Job {
        id: 1,
        name: "Job 1".to_string(),
        description: "Job 1 description".to_string(),
        job_type_id: 1,
    }];
}
