use std::{collections::HashMap, ops::Mul};

use actix_web::{get, web, HttpResponse};
use bigdecimal::{BigDecimal, ToPrimitive};
use tokio::task::JoinSet;

use crate::{
    auth::user_identity::UserIdentity,
    dtos::summary::summary_category_total::CategoryTotalDto,
    entity::{job_rate::JobRate, job_type::JobType},
    repository::{
        job_rate_curve_repository, job_rate_repository, job_repository, job_type_repository,
    },
    AppState,
};

use super::errors::ApiError;

type Result<T> = std::result::Result<T, ApiError>;

#[get("/category")]
async fn get_summary(data: web::Data<AppState>, identity: UserIdentity) -> Result<HttpResponse> {
    let user_id = identity.id;

    let mut result_map = HashMap::<i32, f64>::new();

    let job_types_promise = job_type_repository::get_all_async(&data.db, user_id);
    let jobs = job_repository::get_all_async(&data.db, user_id).await?;

    let promises = jobs
        .iter()
        .map(|job| {
            job_rate_repository::get_by_job_rate_curve_id_async(
                &data.db,
                user_id,
                job.job_rate_curve_id,
            )
        })
        .collect::<Vec<_>>();

    let rates_results = futures::future::join_all(promises).await;

    let mut map: HashMap<i32, Vec<JobRate>> = HashMap::new();

    // Convert the results into a hashmap but fail if any of the results are an error
    for rates_result in rates_results {
        let rates = rates_result?;

        if rates.len() == 0 {
            continue;
        }

        map.insert(rates[0].job_rate_curve_id, rates);
    }

    let job_types = job_types_promise.await?;
    let job_types_map = job_types
        .iter()
        .map(|job_type| (job_type.id, job_type))
        .collect::<HashMap<_, _>>();

    jobs.into_iter().for_each(|job| {
        let rates = map.get(&job.job_rate_curve_id).unwrap();

        let time_in_hours = job
            .end_date
            .unwrap()
            .signed_duration_since(job.start_date.unwrap())
            .num_minutes() as f64
            / 60f64;

        let rate = get_appropriate_rate(rates, time_in_hours);

        let job_type = job_types_map.get(&job.job_type_id).unwrap();

        let total = rate * time_in_hours;

        if result_map.contains_key(&job_type.id) {
            let current_total = result_map.get(&job_type.id).unwrap();

            result_map.insert(job_type.id, current_total + total);
        } else {
            result_map.insert(job_type.id, total);
        }
    });

    let result = result_map
        .into_iter()
        .map(|(key, value)| {
            let job_type = job_types_map.get(&key).unwrap();

            CategoryTotalDto::new(job_type.name.clone(), value)
        })
        .collect::<Vec<_>>();

    return Ok(HttpResponse::Ok().json(result));
}

fn get_appropriate_rate(rates: &Vec<JobRate>, time_in_hours: f64) -> f64 {
    return rates
        .iter()
        .filter(|rate| rate.threshold as f64 <= time_in_hours)
        .max_by_key(|rate| rate.threshold)
        .unwrap()
        .rate
        .to_f64()
        .unwrap();
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("summary").service(get_summary);

    cfg.service(scope);
}
