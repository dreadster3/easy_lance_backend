use super::{job_rate_curves, job_rates, job_types, jobs, users};

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("api")
        .configure(users::register_routes)
        .configure(jobs::register_routes)
        .configure(job_types::register_routes)
        .configure(job_rates::register_routes)
        .configure(job_rate_curves::register_routes);

    cfg.service(scope);
}
