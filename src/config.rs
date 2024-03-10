use std::env;

pub struct Config {
    pub url: String,
    pub timeout: u64,
    pub db_url: String,
    pub db_max_connections: u32,
    pub enable_migrations: bool,
}

impl Config {
    pub fn from_env() -> Self { 
        Config {
            url: env::var("URL").expect("URL must be set"),
            timeout: env::var("TIMEOUT").expect("TIMEOUT must be set").parse().expect("TIMEOUT must be a number"),
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            db_max_connections: env::var("DB_POOL_MAX_CONNECTIONS_THREAD").unwrap_or("1".to_string()).parse().unwrap_or(1),
            enable_migrations: env::var("ENABLE_MIGRATIONS").unwrap_or("false".to_string()).parse().unwrap_or(false),
        }
    }
}
