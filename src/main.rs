use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpServer};
use easy_lance::{api, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init logger
    let log_env = env_logger::Env::new().default_filter_or("info");
    env_logger::init_from_env(log_env);

    // Load .env file
    dotenv::dotenv().ok();

    let app_state = AppState::init().await;

    return HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        return App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(NormalizePath::trim())
            .configure(api::routes::register_routes);
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
}
