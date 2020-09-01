use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

mod config;
mod handlers;

use crate::config::AppConfig;
use crate::handlers::app_config;

#[actix_rt::main]
async fn main() -> Result<()> {
  let conf = AppConfig::from_env()?;

  info!("Server listening at {}:{}", conf.host, conf.port);

  HttpServer::new(move || App::new().wrap(Logger::default()).configure(app_config))
    .bind(format!("{}:{}", conf.host, conf.port))?
    .run()
    .await?;

  Ok(())
}
