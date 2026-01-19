use crate::config::Config;
use resend_rs::Resend;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct SharedState {
  pub config: Config,
  pub db: PgPool,
  pub resend: Resend,
}

impl SharedState {
  pub async fn from_env() -> Self {
    let config = Config::from_env();

    let db = PgPoolOptions::new()
      .max_connections(5)
      .connect(&config.database_url)
      .await
      .unwrap();

    let resend = Resend::new(&config.resend_api_key);

    Self { config, db, resend }
  }
}
