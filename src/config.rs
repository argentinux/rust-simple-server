use color_eyre::Result;
use config::{Config, Environment};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
  pub host: String,
  pub port: i32,
}

impl AppConfig {
  #[instrument]
  pub fn from_env() -> Result<AppConfig> {
    dotenv().ok();

    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();

    info!("Loading config form environment...");

    let mut conf = Config::new();
    conf.merge(Environment::default())?;

    conf.try_into().context("Loading config from environment")
  }
}
