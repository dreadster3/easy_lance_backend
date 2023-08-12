use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CategoryTotalDto {
    pub category: String,
    pub total: f64,
}

impl CategoryTotalDto {
    pub fn new(category: String, total: f64) -> Self {
        Self { category, total }
    }
}
