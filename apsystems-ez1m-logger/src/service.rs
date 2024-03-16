use std::{error::Error, time::Duration};

use apsystems_ez1m_util::db::pool::{initialize_connection_pool, DbPool};
use log::error;
use reqwest::Client;

use crate::{config::Config, model::CurrentOutput};

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
            async move {
                perform_operation(&config, &client, &connection_pool).await;
            }
        });
    }
}

async fn perform_operation(config: &Config, client: &Client, connection_pool: &DbPool) {
    let data = match get_data(config, client).await {
        Some(data) => data,
        None => return,
    };
    write_data_to_db(connection_pool, &data).await;
}

async fn get_data(config: &Config, client: &Client) -> Option<CurrentOutput> {
    let response = client
        .get(format!("{}/getOutputData", config.url).as_str())
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let response = response.json::<CurrentOutput>().await;
            match response {
                Ok(response) => {
                    return Some(response);
                }
                Err(e) => {
                    error!("Failed to parse response: {:?}", e);
                }
            }
        }
        _ => {
            if let Some(device_id) = &config.device_id {
                return Some(CurrentOutput::new(device_id, &String::from("OFFLINE")));
            } else {
                error!(
                    "Failed to get data from the server: {:?}",
                    response.status()
                );
            }
        }
    };

    None
}

async fn write_data_to_db(connection_pool: &DbPool, current_output: &CurrentOutput) {
    let result = sqlx::query(
        r#"
            INSERT INTO ez1m_data(p1, e1, te1, p2, e2, te2, message, device_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
    )
    .bind(current_output.data.p1)
    .bind(current_output.data.e1)
    .bind(current_output.data.te1)
    .bind(current_output.data.p2)
    .bind(current_output.data.e2)
    .bind(current_output.data.te2)
    .bind(current_output.message.as_str())
    .bind(current_output.device_id.as_str())
    .execute(connection_pool)
    .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            error!("Error while creating message topic: {}", e);
        }
    }
}
