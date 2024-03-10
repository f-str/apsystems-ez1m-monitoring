mod config;
mod db;
mod model;
mod service;

use crate::db::migration::run_migrations;
use crate::service::worker_loop;
use crate::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    run_migrations(&config)
        .await
        .expect("Cannot run DB migrations: {}");
    
    worker_loop(&config)
        .await
        .expect("Failed to run worker loop: {}")
}
