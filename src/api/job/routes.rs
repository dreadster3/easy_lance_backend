use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{dtos::job_dto::JobDto, entity::job::Job, repository::job_repository, AppState};

#[get("")]
async fn get_all(data: web::Data<AppState>) -> impl Responder {
    let results = job_repository::get_all_async(&data.db).await;

    return HttpResponse::Ok().json(results);
}

#[post("")]
async fn create(data: web::Data<AppState>, body: web::Json<JobDto>) -> impl Responder {
    let job = Job::from_dto(body.into_inner());

    let result = job_repository::create_async(&data.db, job).await;

    return HttpResponse::Ok().json(result);
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("jobs")
        .service(get_all)
        .service(create);

    cfg.service(scope);
}
