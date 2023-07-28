use serde::Deserialize;

use crate::entity::job::Job;

#[derive(Deserialize)]
pub struct JobDto {
    pub name: String,
    pub description: String,

    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,

    pub job_type_id: i32,
}

impl Into<Job> for JobDto {
    fn into(self) -> crate::entity::job::Job {
        Job::new(
            self.name,
            self.description,
            self.start_date,
            self.end_date,
            self.job_type_id,
        )
    }
}
