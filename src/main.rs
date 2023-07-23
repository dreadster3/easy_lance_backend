mod app;

use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_env = env_logger::Env::new()
                    .default_filter_or("info");

    env_logger::init_from_env(log_env);

    return HttpServer::new( || {
        return App::new()
            .wrap(Logger::default())
            .configure(app::routes::register_routes);
    })
    .bind(("127.0.0.1", 8080))?
    .run().await;
}
