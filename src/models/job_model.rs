use crate::entity::job::Job;

use super::{job_rate_model::JobRateModel, job_type_model::JobTypeModel};

#[derive(Debug, serde::Serialize)]
pub struct JobModel {
    pub id: i32,
    pub name: String,
    pub description: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub start_date: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub end_date: chrono::DateTime<chrono::Utc>,

    pub job_type_id: i32,
    pub job_type: Option<JobTypeModel>,

    pub job_rate_curve_id: i32,
    pub job_rate: Option<JobRateModel>,
}

impl JobModel {
    pub fn get_duration(&self) -> chrono::Duration {
        return self.end_date - self.start_date;
    }

    pub fn get_total(&self) -> f64 {
        let duration = self.get_duration();

        let hours = duration.num_seconds() as f64 / 3600.0;

        let rate = match self.job_rate {
            Some(rate) => rate,
            None => return 0.0,
        };

        return hours * rate.rate;
    }
}

impl From<Job> for JobModel {
    fn from(job: Job) -> Self {
        JobModel {
            id: job.id,
            name: job.name,
            description: job.description,
            start_date: job.start_date.unwrap(),
            end_date: job.end_date.unwrap(),
            job_type_id: job.job_type_id,
            job_type: match job.job_type {
                Some(jt) => Some(jt.into()),
                None => None,
            },
            job_rate_curve_id: job.job_rate_curve_id,
            job_rate: None,
        }
    }
}
