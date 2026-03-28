use crate::config::Config;
use axum::extract::FromRef;
use resend_rs::Resend;
use sqlx::{PgPool, postgres::PgPoolOptions};
use starfish_stripe::Stripe;

#[derive(Debug, Clone, FromRef)]
pub struct SharedState {
  pub config: Config,
  pub db: PgPool,
  pub resend: Resend,
  pub stripe: Stripe,
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
    let stripe = Stripe::new(&config.stripe_secret_key);

    Self {
      config,
      db,
      resend,
      stripe,
    }
  }
}
