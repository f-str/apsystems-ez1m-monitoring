use std::env;

#[derive(Clone)]
pub struct DbConfig {
    pub db_url: String,
    pub db_max_connections: u32,
    pub enable_migrations: bool,
}

impl DbConfig {
    pub fn from_env() -> Self {
        DbConfig {
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            db_max_connections: env::var("DB_POOL_MAX_CONNECTIONS_THREAD")
                .unwrap_or("1".to_string())
                .parse()
                .unwrap_or(1),
            enable_migrations: env::var("ENABLE_MIGRATIONS")
                .unwrap_or("false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}

#[derive(Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        EmailConfig {
            smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"),
            smtp_port: env::var("SMTP_PORT")
                .expect("SMTP_PORT must be set")
                .parse()
                .expect("SMTP_PORT must be a number"),
            smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            smtp_from: env::var("SMTP_FROM").expect("SMTP_FROM must be set"),
        }
    }
}
