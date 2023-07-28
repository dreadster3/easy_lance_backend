use serde::Deserialize;

use crate::entity::job_type::JobType;

#[derive(Deserialize)]
pub struct JobTypeDto {
    pub name: String,
}

impl Into<JobType> for JobTypeDto {
    fn into(self) -> JobType {
        JobType::new(self.name)
    }
}
