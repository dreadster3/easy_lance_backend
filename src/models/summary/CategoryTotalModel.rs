#[derive(Debug, serde::Serialize)]
pub struct CategoryTotalModel {
    pub category: String,
    pub total: f64,
}
