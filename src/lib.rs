pub mod api;
mod dtos;
mod entity;
mod repository;

pub struct AppState {
    pub db: sqlx::Pool<sqlx::Postgres>,
}
