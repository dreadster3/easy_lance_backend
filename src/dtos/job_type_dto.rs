use serde::Deserialize;

#[derive(Deserialize)]
pub struct JobTypeDto {
    pub name: String,
}
