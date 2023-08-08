use chrono;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    #[serde(skip_serializing)]
    pub refresh_token: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing)]
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: 0i32,
            username,
            email,
            password,
            refresh_token: None,
            created_at: Option::from(chrono::Local::now().naive_local()),
            modified_at: Option::from(chrono::Local::now().naive_local()),
        }
    }
}
