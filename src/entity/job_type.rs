use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JobType {
    id: i32,
    name: String,
}
