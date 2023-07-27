use super::{job, job_type};

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("api")
        .configure(job::register_routes)
        .configure(job_type::register_routes);

    cfg.service(scope);
}
