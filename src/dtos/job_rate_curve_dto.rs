use crate::entity::job_rate_curve::JobRateCurve;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JobRateCurveDto {
    pub name: String,
}

impl JobRateCurveDto {
    pub fn to_entity(self, user_id: i32) -> JobRateCurve {
        JobRateCurve::new(user_id, self.name)
    }
}
