use std::collections::HashMap;

use crate::{
    entity::job_rate::JobRate,
    models::job_model::JobModel,
    repository::{job_rate_repository, job_repository},
};

use super::errors::ServiceError;

type Result<T> = std::result::Result<T, ServiceError>;

pub async fn get_all_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobModel>> {
    let result = job_repository::get_all_async(pool, user_id).await?;

    return result;
}

pub async fn get_all_full_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<JobModel>> {
    let result = job_repository::get_all_async(pool, user_id).await?;

    let promises = result
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

    let mut map: HashMap<i32, Vec<JobRate>> = HashMap::new();

    // Convert the results into a hashmap but fail if any of the results are an error
    for rates_result in rates_results {
        let rates = rates_result?;

        if rates.len() == 0 {
            continue;
        }

        map.insert(rates[0].job_rate_curve_id, rates);
    }

    return result;
}
