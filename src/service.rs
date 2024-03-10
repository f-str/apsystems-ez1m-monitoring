use std::error::Error;

use log::error;
use reqwest::Client;

use crate::{
    config::Config,
    db::pool::{initialize_connection_pool, DbPool},
    model::CurrentOutput,
};

pub async fn worker_loop(config: &Config) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let connection_pool = initialize_connection_pool(config).await.unwrap();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(config.timeout));
        let data = match get_data(config, &client).await {
            Some(data) => data,
            None => continue,
        };
        write_data_to_db(&connection_pool, &data).await;
    }
}

async fn get_data(config: &Config, client: &Client) -> Option<CurrentOutput> {
    let response = client
        .get(format!("{}/data", config.url).as_str())
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
        _ => error!("Failed to get data: {:?}", response.status()),
    };

    None
}

async fn write_data_to_db(connection_pool: &DbPool, current_output: &CurrentOutput) {
    let result = sqlx::query(
        r#"
            INSERT INTO ez1m_data(p1, e1, t1, p2, e2, t2, message, device_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
    )
    .bind(current_output.data.p1)
    .bind(current_output.data.e1)
    .bind(current_output.data.t1)
    .bind(current_output.data.p2)
    .bind(current_output.data.e2)
    .bind(current_output.data.t2)
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
