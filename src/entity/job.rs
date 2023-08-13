use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, Row};

use super::{job_type::JobType, traits::FromRowPrefixed};

#[derive(Debug, Serialize)]
pub struct Job {
    pub id: i32,

    pub name: String,

    pub description: String,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing)]
    pub modified_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: Option<chrono::NaiveDateTime>,

    #[serde(skip_serializing)]
    pub user_id: i32,

    pub job_rate_curve_id: i32,

    pub job_type_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<JobType>,
}

impl FromRow<'_, PgRow> for Job {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        return Self::from_row_prefixed(row, "");
    }
}
impl FromRowPrefixed<'_, PgRow> for Job {
    fn from_row_prefixed(row: &'_ PgRow, prefix: &str) -> Result<Self, sqlx::Error> {
        let prefix = match prefix.len() {
            0 => "".to_string(),
            _ => String::from(prefix) + "_",
        };

        Ok(Self {
            id: row.try_get(format!("{}id", prefix).as_str())?,
            name: row.try_get(format!("{}name", prefix).as_str())?,
            description: row.try_get(format!("{}description", prefix).as_str())?,
            start_date: row.try_get(format!("{}start_date", prefix).as_str())?,
            end_date: row.try_get(format!("{}end_date", prefix).as_str())?,
            modified_at: row.try_get(format!("{}modified_at", prefix).as_str())?,
            created_at: row.try_get(format!("{}created_at", prefix).as_str())?,
            user_id: row.try_get(format!("{}user_id", prefix).as_str())?,
            job_type_id: row.try_get(format!("{}job_type_id", prefix).as_str())?,
            job_type: match JobType::from_row_prefixed(row, "job_type") {
                Ok(jt) => Some(jt),
                Err(_) => None,
            },
            job_rate_curve_id: row.try_get(format!("{}job_rate_curve_id", prefix).as_str())?,
        })
    }
}

impl Job {
    pub fn new(
        user_id: i32,
        name: String,
        description: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        job_type_id: i32,
        job_rate_curve_id: i32,
    ) -> Self {
        Self {
            id: 0i32,
            name,
            description,
            user_id,
            start_date: Option::from(start_date),
            end_date: Option::from(end_date),
            created_at: Option::from(chrono::Local::now().naive_local()),
            modified_at: Option::from(chrono::Local::now().naive_local()),
            job_type_id,
            job_type: None,
            job_rate_curve_id,
        }
    }
}
