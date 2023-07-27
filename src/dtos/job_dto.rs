use serde::Deserialize;

#[derive(Deserialize)]
pub struct JobDto {
    pub name: String,
    pub description: String,

    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,

    pub job_type_id: i32,
}
