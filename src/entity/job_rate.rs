use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JobRate {
    pub id: i32,
    pub rate: BigDecimal,
    pub threshold: i32,

    #[serde(skip_serializing)]
    pub user_id: i32,

    #[serde(skip_serializing)]
    pub modified_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl JobRate {
    pub fn new(user_id: i32, rate: BigDecimal, threshold: i32) -> Self {
        Self {
            id: 0i32,
            rate,
            threshold,
            user_id: 0i32,
            created_at: Option::from(chrono::Local::now().naive_local()),
            modified_at: Option::from(chrono::Local::now().naive_local()),
        }
    }
}
