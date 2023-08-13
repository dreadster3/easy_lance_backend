use std::collections::HashMap;

use chrono::Duration;

use crate::{
    entity::job_rate::JobRate,
    models::{job_model::JobModel, job_rate_model::JobRateModel},
    repository::{job_rate_repository, job_repository},
};

use super::errors::ServiceError;

type Result<T> = std::result::Result<T, ServiceError>;

pub async fn get_all_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobModel>> {
    let jobs = job_repository::get_all_async(pool, user_id).await?;

    let result = jobs
        .into_iter()
        .map(|job| JobModel::from(job))
        .collect::<Vec<_>>();

    return Ok(result);
}

pub async fn get_all_full_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobModel>> {
    let jobs = job_repository::get_all_with_type_async(pool, user_id).await?;

    let promises = jobs
        .iter()
        .map(|job| {
            job_rate_repository::get_by_job_rate_curve_id_async(
                pool,
                user_id,
                job.job_rate_curve_id,
            )
        })
        .collect::<Vec<_>>();

    let rates_results = futures::future::join_all(promises).await;

    let mut map = HashMap::<i32, Vec<JobRateModel>>::new();

    for rates_result in rates_results {
        let rates = rates_result?
            .into_iter()
            .map(|rate| JobRateModel::from(rate))
            .collect::<Vec<_>>();

        if rates.len() == 0 {
            continue;
        }

        map.insert(rates[0].job_rate_curve_id, rates);
    }

    let result = jobs
        .into_iter()
        .map(|job| {
            let mut job_model = JobModel::from(job);

            let rates = map.get(&job_model.job_rate_curve_id).unwrap();

            let duration = job_model
                .end_date
                .signed_duration_since(job_model.start_date);

            let rate = get_appropriate_rate(rates, duration);

            job_model.job_rate = Some(rate);

            return job_model;
        })
        .collect::<Vec<_>>();

    return Ok(result);
}

fn get_appropriate_rate(rates: &Vec<JobRateModel>, duration: Duration) -> JobRateModel {
    let hours = duration.num_seconds() as f64 / 3600.0;

    let rate = rates
        .into_iter()
        .filter(|r| r.threshold as f64 <= hours)
        .max_by_key(|r| r.threshold)
        .unwrap();

    return rate.clone();
}
