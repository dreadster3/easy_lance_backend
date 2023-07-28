use serde::Deserialize;

use crate::entity::job_type::JobType;

#[derive(Deserialize)]
pub struct JobTypeDto {
    pub name: String,
}

impl JobTypeDto {
    pub fn to_entity(self, user_id: i32) -> JobType {
        JobType::new(user_id, self.name)
    }
}
