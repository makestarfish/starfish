use serde::Deserialize;

fn default_port() -> u16 {
  3333
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  #[serde(default = "default_port")]
  pub port: u16,
  pub database_url: String,
  pub resend_api_key: String,
  pub stripe_secret_key: String,
  pub stripe_webhook_signing_secret: String,
  pub website_base_url: String,
  pub jwt_secret: String,
}

impl Config {
  pub fn from_env() -> Self {
    envy::from_env::<Config>().expect("invalid environment variables")
  }
}
