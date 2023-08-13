use bigdecimal::ToPrimitive;

use crate::entity::job_rate::JobRate;

#[derive(Debug, serde::Serialize, Copy, Clone)]
pub struct JobRateModel {
    pub id: i32,
    pub rate: f64,
    pub threshold: i32,

    pub job_rate_curve_id: i32,
}

impl From<JobRate> for JobRateModel {
    fn from(job_rate: JobRate) -> Self {
        JobRateModel {
            id: job_rate.id,
            rate: job_rate.rate.to_f64().unwrap(),
            threshold: job_rate.threshold,
            job_rate_curve_id: job_rate.job_rate_curve_id,
        }
    }
}
