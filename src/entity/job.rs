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
    pub created_at: Option<chrono::NaiveDateTime>,

    pub job_type_id: i32,
}

impl Job {
    pub fn new(
        name: String,
        description: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        job_type_id: i32,
    ) -> Self {
        Self {
            id: 0i32,
            name,
            description,
            start_date: Option::from(start_date),
            end_date: Option::from(end_date),
            created_at: Option::from(chrono::Local::now().naive_local()),
            job_type_id,
        }
    }

    pub fn from_dto(dto: crate::dtos::job_dto::JobDto) -> Self {
        Job::new(
            dto.name,
            dto.description,
            chrono::Utc::now(),
            chrono::Utc::now(),
            dto.job_type_id,
        )
    }
}
