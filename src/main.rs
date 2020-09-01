use color_eyre::Result;

mod config;

use crate::config::AppConfig;

#[actix_rt::main]
async fn main() -> Result<()> {
  let conf = AppConfig::from_env()?;

  println!("{:?}", conf);

  Ok(())
}
