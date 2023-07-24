mod api;
mod entity;
mod repository;

use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpServer};

pub struct AppState {
    db: sqlx::Pool<sqlx::Postgres>,
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init logger
    let log_env = env_logger::Env::new().default_filter_or("info");
    env_logger::init_from_env(log_env);

    // Load .env file
    dotenv::dotenv().ok();

    // Connect to database
    let database_url = get_database_url();
    let pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
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

    return HttpServer::new(move || {
        return App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .configure(api::routes::register_routes);
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
}
