use serde::Deserialize;

use crate::entity::job::Job;

#[derive(Deserialize)]
pub struct JobDto {
    pub name: String,
    pub description: String,

    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,

    pub job_type_id: i32,

    pub job_rate_curve_id: i32,
}

impl JobDto {
    pub fn to_entity(self, user_id: i32) -> Job {
        Job::new(
            user_id,
            self.name,
            self.description,
            self.start_date,
            self.end_date,
            self.job_type_id,
            self.job_rate_curve_id,
        )
    }
}
