use actix_web::{get, web, HttpResponse, Responder};

use crate::{repository::job_repository, AppState};

#[get("")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let results = job_repository::get_all_async(&data.db).await;

    return HttpResponse::Ok().json(results);
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("jobs").service(index);

    cfg.service(scope);
}
