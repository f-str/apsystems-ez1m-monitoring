use log::{error, info};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::config::Config;

pub type DbPool = Pool<Postgres>;

pub async fn initialize_connection_pool(config: &Config) -> Option<DbPool> {
    match get_pool(config).await {
        Ok(pool) => {
            info!("DB connection pool initialized!");
            Some(pool)
        }
        Err(e) => {
            error!("DB connection pool could not be initialized: {}", e);
            None
        }
    }
}

async fn get_pool(config: &Config) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(config.db_url.as_str())
        .await
}
