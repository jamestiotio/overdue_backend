use config::ConfigError;
use deadpool_postgres::Pool;
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
        .add_source(
            config::Environment::default()
            .separator("__")
        )
        .build()?
        .try_deserialize()
    }

    pub fn configure_pool(&self) -> Pool {
        self.pg
            .create_pool(NoTls)
            .expect("error creating deadpool postgres database pool")
    }
}
