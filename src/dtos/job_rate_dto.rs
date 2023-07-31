use crate::entity::job_rate::JobRate;
use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JobRateDto {
    pub rate: BigDecimal,
    pub threshold: i32,
    pub job_rate_curve_id: i32,
}

impl JobRateDto {
    pub fn to_entity(self, user_id: i32) -> JobRate {
        JobRate::new(user_id, self.job_rate_curve_id, self.rate, self.threshold)
    }
}
