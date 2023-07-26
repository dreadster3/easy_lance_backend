use serde::Deserialize;

#[derive(Deserialize)]
pub struct JobDto {
    pub name: String,
    pub description: String,
    pub job_type_id: i32,
}
