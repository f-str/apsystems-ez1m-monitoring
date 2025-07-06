use std::{error::Error, time::Duration};

use apsystems_ez1m_util::db::pool::{initialize_connection_pool, DbPool};
use log::warn;
use reqwest::Client;
use thiserror::Error;

use crate::{config::Config, model::CurrentOutput};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Failed to parse response from device to model.")]
    ResponseParseError,
    #[error("Failed to reach host.")]
    HostNotReachableError,
}

pub async fn worker_loop(config: &Config) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let connection_pool = initialize_connection_pool(&config.db_config).await.unwrap();
    let mut interval = tokio::time::interval(Duration::from_secs(config.timeout));

    loop {
        interval.tick().await;

        // Spawn the perform_operation function in a new Tokio task
        tokio::spawn({
            let config = config.clone();
            let client = client.clone();
            let connection_pool = connection_pool.clone();
            async move{
                match perform_operation(&config, &client, &connection_pool).await {
                    Ok(_) => (),
                    Err(_e) => warn!("{_e}")
                }
            }
        });
    }
}

async fn perform_operation(config: &Config, client: &Client, connection_pool: &DbPool) -> Result<(), Box<dyn Error>> {
    let data = match get_data(config, client).await? {
        Some(data) => data,
        None => return Ok(()),
    };
    let _ = write_data_to_db(connection_pool, &data).await?;
    Ok(())
}

async fn get_data(config: &Config, client: &Client) -> Result<Option<CurrentOutput>, ServiceError> {
    let response = client
        .get(format!("{}/getOutputData", config.url).as_str())
        .send()
        .await.map_err(|_e| ServiceError::HostNotReachableError)?;

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<CurrentOutput>().await {
                Ok(data) => Ok(Some(data)),
                Err(_e) => Err(ServiceError::ResponseParseError),
            }
        }
        _ => Ok(None)
    }
}

async fn write_data_to_db(connection_pool: &DbPool, current_output: &CurrentOutput) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
            INSERT INTO data_ez1m(device_id, value_key, double_value)
            VALUES 
                ($1, 'p1', $2), 
                ($1, 'e1', $3), 
                ($1, 'te1', $4), 
                ($1, 'p2', $5), 
                ($1, 'e2', $6), 
                ($1, 'te2', $7);
            "#,
    )
    .bind(current_output.device_id.as_str())
    .bind(current_output.data.p1)
    .bind(current_output.data.e1)
    .bind(current_output.data.te1)
    .bind(current_output.data.p2)
    .bind(current_output.data.e2)
    .bind(current_output.data.te2)
    .execute(connection_pool)
    .await
}
