use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JobType {
    id: u32,
    name: String,
}
