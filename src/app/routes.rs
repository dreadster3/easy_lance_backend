use super::animals;

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("api")
        .configure(animals::routes::register_routes);

    cfg.service(scope);
}
