use std::env;

use apsystems_ez1m_util::config::DbConfig;

#[derive(Clone)]
pub struct Config {
    pub url: String,
    pub timeout: u64,
    pub db_config: DbConfig,
    pub device_id: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            url: env::var("URL").expect("URL must be set"),
            timeout: env::var("TIMEOUT")
                .expect("TIMEOUT must be set")
                .parse()
                .expect("TIMEOUT must be a number"),
            db_config: DbConfig::from_env(),
            device_id: match env::var("DEVICE_ID") {
                Ok(device_id) => Some(device_id),
                Err(_) => None,
            },
        }
    }
}
