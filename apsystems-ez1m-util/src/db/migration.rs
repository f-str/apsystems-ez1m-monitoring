use std::error::Error;

use crate::config::DbConfig;
use log::{error, info};
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub async fn run_migrations(config: &DbConfig) -> Result<(), Box<dyn Error>> {
    if !config.enable_migrations {
        info!("Skipping DB migrations...");
        return Ok(());
    }

    info!("Running DB migrations...");

    let (mut client, con) = tokio_postgres::connect(config.db_url.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = con.await {
            error!("connection error: {e}");
        }
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    for migration in migration_report.applied_migrations() {
        info!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    info!("DB migrations finished!");

    Ok(())
}
