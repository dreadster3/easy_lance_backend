use actix_web::{get, HttpResponse, Responder};
use log::info;

#[get("/")]
pub async fn index() -> impl Responder {
    info!("Hello from animals::routes::index()!");
    return HttpResponse::Ok().body("Hello world!");
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("animals")
        .service(index);

    cfg.service(scope);
}
