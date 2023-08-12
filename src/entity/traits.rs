use sqlx::{FromRow, Row};

pub trait FromRowPrefixed<'r, R: Row>: FromRow<'r, R> {
    fn from_row_prefixed(row: &'r R, prefix: &str) -> Result<Self, sqlx::Error>;
}
