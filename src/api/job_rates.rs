use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};

use crate::{
    auth::user_identity::UserIdentity, dtos::job_rate_dto::JobRateDto,
    repository::job_rates_repository, AppState,
};

use super::errors::ApiError;

type Result<T> = std::result::Result<T, ApiError>;

#[get("")]
async fn get_all(data: web::Data<AppState>, identity: UserIdentity) -> Result<HttpResponse> {
    let result = job_rates_repository::get_all_async(&data.db, identity.id).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[get("/{id}")]
async fn get_by_id(
    data: web::Data<AppState>,
    identity: UserIdentity,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let result =
        job_rates_repository::get_by_id_async(&data.db, identity.id, id.into_inner()).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[post("")]
async fn create(
    data: web::Data<AppState>,
    identity: UserIdentity,
    body: Json<JobRateDto>,
) -> Result<HttpResponse> {
    let user_id = identity.id;
    let job_rate = body.into_inner().to_entity(user_id);

    let result = job_rates_repository::create_async(&data.db, user_id, job_rate).await?;

    return Ok(HttpResponse::Created().json(result));
}

#[put("/{id}")]
async fn update(
    data: web::Data<AppState>,
    identity: UserIdentity,
    id: web::Path<i32>,
    body: Json<JobRateDto>,
) -> Result<HttpResponse> {
    let user_id = identity.id;
    let job_rate = body.into_inner().to_entity(user_id);

    let result =
        job_rates_repository::update_async(&data.db, user_id, id.into_inner(), job_rate).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[delete("/{id}")]
async fn delete(
    data: web::Data<AppState>,
    identity: UserIdentity,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    job_rates_repository::delete_async(&data.db, identity.id, id.into_inner()).await?;

    return Ok(HttpResponse::NoContent().finish());
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("jobrates")
        .service(get_all)
        .service(get_by_id)
        .service(create)
        .service(update)
        .service(delete);

    cfg.service(scope);
}
