use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Job {
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

    pub job_rate_id: i32,

    pub job_type_id: i32,
}

impl Job {
    pub fn new(
        user_id: i32,
        name: String,
        description: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        job_type_id: i32,
        job_rate_id: i32,
    ) -> Self {
        Self {
            id: 0i32,
            name,
            description,
            user_id,
            start_date: Option::from(start_date),
            end_date: Option::from(end_date),
            created_at: Option::from(chrono::Local::now().naive_local()),
            modified_at: Option::from(chrono::Local::now().naive_local()),
            job_type_id,
            job_rate_id,
        }
    }
}
