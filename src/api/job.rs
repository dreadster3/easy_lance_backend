use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::{
    api::errors::ApiError, dtos::job_dto::JobDto, entity::job::Job, repository::job_repository,
    AppState,
};

type Result<T> = std::result::Result<T, ApiError>;

#[get("")]
async fn get_all(data: web::Data<AppState>) -> Result<HttpResponse> {
    let result = job_repository::get_all_async(&data.db).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[get("/{id}")]
async fn get_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse> {
    let result = job_repository::get_by_id_async(&data.db, id.into_inner()).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[post("")]
async fn create(data: web::Data<AppState>, body: web::Json<JobDto>) -> Result<HttpResponse> {
    let job: Job = body.into_inner().into();

    let result = job_repository::create_async(&data.db, job).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[put("/{id}")]
async fn update(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<JobDto>,
) -> Result<HttpResponse> {
    let job_id = id.into_inner();
    let job: Job = body.into_inner().into();

    let result = job_repository::update_async(&data.db, job_id, job).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[delete("/{id}")]
async fn delete(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse> {
    let job_id = id.into_inner();

    job_repository::delete_async(&data.db, job_id).await?;

    return Ok(HttpResponse::NoContent().finish());
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("jobs")
        .service(get_all)
        .service(get_by_id)
        .service(create)
        .service(update)
        .service(delete);

    cfg.service(scope);
}
