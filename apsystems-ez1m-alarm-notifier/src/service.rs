use std::{error::Error, time::Duration};

use apsystems_ez1m_util::{
    db::pool::{initialize_connection_pool, DbPool},
    email::send_emails,
};
use log::error;
use reqwest::Client;
use sqlx::Row;

use crate::{
    config::Config,
    model::Alarm,
    templates::{
        DC1_SHORT_CIRCUIT_ALARM_TEMPLATE, DC2_SHORT_CIRCUIT_ALARM_TEMPLATE, OFFGRID_ALARM_TEMPLATE,
        OUTPUT_FAULT_ALARM_TEMPLATE,
    },
};

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
                let _ = perform_operation(&config, &client, &connection_pool).await;
            }
        });
    }
}

async fn perform_operation(
    config: &Config,
    client: &Client,
    connection_pool: &DbPool,
) -> Result<(), sqlx::Error> {
    match get_data(config, client).await {
        Some(data) => {
            let data = data.data;

            let mut recipients = Vec::new();

            if data.og | data.oe | data.isce1 | data.isce2 == 1 {
                recipients = get_recipients(connection_pool).await?;
            }

            if data.og == 1 {
                let subject = "Off Grid Alarm".to_string();
                let _ = send_emails(
                    &config.email_config,
                    &subject,
                    OFFGRID_ALARM_TEMPLATE,
                    &recipients,
                )
                .await;
            }

            if data.oe == 1 {
                let subject = "Output Fault Alarm".to_string();
                let _ = send_emails(
                    &config.email_config,
                    &subject,
                    OUTPUT_FAULT_ALARM_TEMPLATE,
                    &recipients,
                )
                .await;
            }

            if data.isce1 == 1 {
                let subject = "DC1 Short Circuit Alarm".to_string();
                let _ = send_emails(
                    &config.email_config,
                    &subject,
                    DC1_SHORT_CIRCUIT_ALARM_TEMPLATE,
                    &recipients,
                )
                .await;
            }

            if data.isce2 == 1 {
                let subject = "DC2 Short Circuit Alarm".to_string();
                let _ = send_emails(
                    &config.email_config,
                    &subject,
                    DC2_SHORT_CIRCUIT_ALARM_TEMPLATE,
                    &recipients,
                )
                .await;
            }
        }
        None => return Ok(()),
    };
    Ok(())
}

async fn get_data(config: &Config, client: &Client) -> Option<Alarm> {
    let response = client
        .get(format!("{}/getAlarm", config.url).as_str())
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let response = response.json::<Alarm>().await;
            match response {
                Ok(response) => {
                    return Some(response);
                }
                Err(e) => {
                    error!("Failed to parse response: {:?}", e);
                }
            }
        }
        _ => error!(
            "Failed to get data from the server: {:?}",
            response.status()
        ),
    };

    None
}

async fn get_recipients(connection_pool: &DbPool) -> Result<Vec<String>, sqlx::Error> {
    Ok(sqlx::query(r#"SELECT email FROM notification_subscriber;"#)
        .fetch_all(connection_pool)
        .await?
        .iter()
        .map(|row| row.get::<String, _>("email"))
        .collect())
}
