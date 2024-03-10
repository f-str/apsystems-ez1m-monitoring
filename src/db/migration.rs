use std::error::Error;

use crate::config::Config;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn run_migrations(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.enable_migrations {
        println!("Skipping DB migrations...");
        return Ok(());
    }

    println!("Running DB migrations...");

    let (mut client, con) = tokio_postgres::connect(config.db_url.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = con.await {
            eprintln!("connection error: {}", e);
        }
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    println!("DB migrations finished!");

    Ok(())
}
