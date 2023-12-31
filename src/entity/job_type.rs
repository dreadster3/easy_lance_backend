use serde::Serialize;



#[derive(Serialize, sqlx::FromRow)]
pub struct JobType {
    pub id: i32,

    pub name: String,

    #[serde(skip_serializing)]
    pub user_id: i32,

    #[serde(skip_serializing)]
    pub modified_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl JobType {
    pub fn new(user_id: i32, name: String) -> Self {
        Self {
            id: 0i32,
            name,
            user_id,
            created_at: Option::from(chrono::Local::now().naive_local()),
            modified_at: Option::from(chrono::Local::now().naive_local()),
        }
    }
}
