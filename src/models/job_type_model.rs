use crate::entity::job_type::JobType;

#[derive(Debug, serde::Serialize)]
pub struct JobTypeModel {
    pub id: i32,
    pub name: String,
}

impl From<JobType> for JobTypeModel {
    fn from(job_type: JobType) -> Self {
        JobTypeModel {
            id: job_type.id,
            name: job_type.name,
        }
    }
}
