use super::job;

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("api").configure(job::routes::register_routes);

    cfg.service(scope);
}
