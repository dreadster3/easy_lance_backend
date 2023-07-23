use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub job_type_id: u32,
}
