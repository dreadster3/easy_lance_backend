pub mod api;
mod auth;
mod dtos;
mod entity;
mod repository;

fn get_database_url() -> String {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(_) => {
            log::error!("DATABASE_URL environment variable is not set");
            std::process::exit(1);
        }
    };

    log::debug!("Database URL: {}", database_url);

    return database_url;
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::Pool<sqlx::Postgres>,
    pub jwt: auth::jwt_configuration::JwtConfiguration,
}

impl AppState {
    pub async fn init() -> Self {
        let db = match sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&get_database_url())
            .await
        {
            Ok(pool) => {
                log::info!("Successfully connected to database");
                pool
            }
            Err(e) => {
                log::error!("Failed to connect to database: {}", e);
                std::process::exit(1);
            }
        };

        let jwt = auth::jwt_configuration::JwtConfiguration {
            secret: std::env::var("JWT_SECRET").unwrap(),
            expiration: std::env::var("JWT_EXPIRATION")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        };

        Self { db, jwt }
    }
}
