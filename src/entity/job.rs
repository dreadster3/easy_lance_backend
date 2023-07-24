use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub job_type_id: i32,
}
