use config::Config;

mod config;
mod model;
mod service;
mod templates;

use apsystems_ez1m_util::db::migration::run_migrations;

use crate::service::worker_loop;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    run_migrations(&config.db_config)
        .await
        .expect("Cannot run DB migrations: {}");

    worker_loop(&config)
        .await
        .expect("Failed to run worker loop: {}");
}
