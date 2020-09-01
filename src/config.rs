use color_eyre::Result;
use config::{Config, Environment};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
  pub host: String,
  pub port: i32,
}

impl AppConfig {
  pub fn from_env() -> Result<AppConfig> {
    dotenv().ok();

    let mut conf = Config::new();
    conf.merge(Environment::default())?;

    conf.try_into().context("Loading config from environment")
  }
}
