use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse,
};

use crate::{
    dtos::job_type_dto::JobTypeDto, entity::job_type::JobType, repository::job_type_repository,
    AppState,
};

use super::errors::ApiError;

type Result<T> = std::result::Result<T, ApiError>;

#[get("")]
async fn get_all_async(data: web::Data<AppState>) -> Result<HttpResponse> {
    let result = job_type_repository::get_all_async(&data.db).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[get("/{id}")]
async fn get_by_id_async(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse> {
    let result = job_type_repository::get_by_id_async(&data.db, id.into_inner()).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[post("")]
async fn create_async(data: web::Data<AppState>, body: Json<JobTypeDto>) -> Result<HttpResponse> {
    let job_type: JobType = body.into_inner().into();

    if job_type_repository::check_duplicate_by_name(&data.db, &job_type.name).await? {
        return Err(ApiError::DuplicateError("JobType".to_string()));
    }

    let result = job_type_repository::create_async(&data.db, job_type).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[put("/{id}")]
async fn update_async(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: Json<JobTypeDto>,
) -> Result<HttpResponse> {
    let job_type_id = id.into_inner();
    let job_type: JobType = body.into_inner().into();

    match job_type_repository::get_by_id_async(&data.db, job_type_id).await {
        Ok(j) => j,
        Err(err) => return Err(ApiError::from(err)),
    };

    if job_type_repository::check_duplicate_by_name(&data.db, &job_type.name).await? {
        return Err(ApiError::DuplicateError("JobType".to_string()));
    }

    let result = job_type_repository::update_async(&data.db, job_type_id, job_type).await?;

    return Ok(HttpResponse::Ok().json(result));
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("jobtypes")
        .service(get_all_async)
        .service(get_by_id_async)
        .service(create_async)
        .service(update_async);

    cfg.service(scope);
}
