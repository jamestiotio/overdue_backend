use deadpool_postgres::Pool;
use serde::Deserialize;
use config::ConfigError;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();

        cfg.merge(config::Environment::new().separator("__"))?;
        
        cfg.try_into()
    }

    pub fn configure_pool(&self) -> Pool {
        self.pg.create_pool(NoTls).expect("error creating deadpool postgres database pool")
    }
}