use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};

use crate::{
    auth::user_identity::UserIdentity, dtos::job_type_dto::JobTypeDto, entity::job_type::JobType,
    repository::job_type_repository, AppState,
};

use super::errors::ApiError;

type Result<T> = std::result::Result<T, ApiError>;

#[get("")]
async fn get_all(data: web::Data<AppState>, identity: UserIdentity) -> Result<HttpResponse> {
    let result = job_type_repository::get_all_async(&data.db, identity.id).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[get("/{id}")]
async fn get_by_id(
    data: web::Data<AppState>,
    identity: UserIdentity,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let result =
        job_type_repository::get_by_id_async(&data.db, identity.id, id.into_inner()).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[post("")]
async fn create(
    data: web::Data<AppState>,
    identity: UserIdentity,
    body: Json<JobTypeDto>,
) -> Result<HttpResponse> {
    let job_type = body.into_inner().to_entity(identity.id);

    if job_type_repository::check_exists_by_name_async(&data.db, identity.id, &job_type.name)
        .await?
    {
        return Err(ApiError::DuplicateError("JobType".to_string()));
    }

    let result = job_type_repository::create_async(&data.db, job_type).await?;

    return Ok(HttpResponse::Created().json(result));
}

#[put("/{id}")]
async fn update(
    data: web::Data<AppState>,
    identity: UserIdentity,
    id: web::Path<i32>,
    body: Json<JobTypeDto>,
) -> Result<HttpResponse> {
    let job_type_id = id.into_inner();
    let job_type: JobType = body.into_inner().to_entity(identity.id);

    if job_type_repository::check_exists_by_name_async(&data.db, identity.id, &job_type.name)
        .await?
    {
        return Err(ApiError::DuplicateError("JobType".to_string()));
    }

    let result = job_type_repository::update_async(&data.db, job_type_id, job_type).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[delete("/{id}")]
async fn delete(
    data: web::Data<AppState>,
    identity: UserIdentity,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let job_type_id = id.into_inner();

    job_type_repository::delete_async(&data.db, identity.id, job_type_id).await?;

    return Ok(HttpResponse::NoContent().finish());
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("jobtypes")
        .service(get_all)
        .service(get_by_id)
        .service(create)
        .service(update)
        .service(delete);

    cfg.service(scope);
}
