use actix_web::{get, web, HttpResponse};
use itertools::Itertools;

use crate::{
    auth::user_identity::UserIdentity, models::summary::CategoryTotalModel::CategoryTotalModel,
    service::job_service, AppState,
};

use super::errors::ApiError;

type Result<T> = std::result::Result<T, ApiError>;

#[get("/category")]
async fn get_category_summary(
    data: web::Data<AppState>,
    identity: UserIdentity,
) -> Result<HttpResponse> {
    let user_id = identity.id;
    let jobs = job_service::get_all_full_async(&data.db, user_id).await?;

    let result = jobs
        .into_iter()
        .group_by(|job| job.job_type_id)
        .into_iter()
        .map(|(_, value)| {
            let vec = value.collect::<Vec<_>>();

            if let Some(first_job) = vec.first() {
                let category = first_job.job_type.as_ref().unwrap().name.clone();
                let total = vec.iter().fold(0.0, |acc, job| acc + job.get_total());

                return CategoryTotalModel { category, total };
            }

            return CategoryTotalModel {
                category: "".to_string(),
                total: 0.0,
            };
        })
        .collect::<Vec<_>>();

    return Ok(HttpResponse::Ok().json(result));
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("summary").service(get_category_summary);

    cfg.service(scope);
}
