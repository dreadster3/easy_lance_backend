use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, Row};

use super::traits::FromRowPrefixed;

#[derive(Debug, Serialize)]
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

impl FromRow<'_, PgRow> for JobType {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        return Self::from_row_prefixed(row, "");
    }
}

impl FromRowPrefixed<'_, PgRow> for JobType {
    fn from_row_prefixed(row: &'_ PgRow, prefix: &str) -> Result<Self, sqlx::Error> {
        let prefix = match prefix.len() {
            0 => "".to_string(),
            _ => String::from(prefix) + "_",
        };

        Ok(Self {
            id: row.try_get(format!("{}id", prefix).as_str())?,
            name: row.try_get(format!("{}name", prefix).as_str())?,
            modified_at: row.try_get(format!("{}modified_at", prefix).as_str())?,
            created_at: row.try_get(format!("{}created_at", prefix).as_str())?,
            user_id: row.try_get(format!("{}user_id", prefix).as_str())?,
        })
    }
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
