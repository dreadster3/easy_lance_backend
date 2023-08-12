use crate::entity::{job::Job, job_rate::JobRate, job_type::JobType};

#[derive(Debug, serde::Serialize)]
pub struct JobModel {
    pub id: i32,

    pub name: String,

    pub description: String,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing)]
    pub modified_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: Option<chrono::NaiveDateTime>,

    #[serde(skip_serializing)]
    pub user_id: i32,

    pub job_rate_curve_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_rate: Option<JobRate>,

    pub job_type_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<JobType>,
}

impl From<Job> for JobModel {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            name: job.name,
            description: job.description,
            start_date: job.start_date,
            end_date: job.end_date,
            modified_at: job.modified_at,
            created_at: job.created_at,
            user_id: job.user_id,
            job_rate_curve_id: job.job_rate_curve_id,
            job_rate: None,
            job_type_id: job.job_type_id,
            job_type: None,
        }
    }
}

impl Into<Job> for JobModel {
    fn into(self) -> Job {
        Job {
            id: self.id,
            name: self.name,
            description: self.description,
            start_date: self.start_date,
            end_date: self.end_date,
            modified_at: self.modified_at,
            created_at: self.created_at,
            user_id: self.user_id,
            job_rate_curve_id: self.job_rate_curve_id,
            job_type_id: self.job_type_id,
        }
    }
}
