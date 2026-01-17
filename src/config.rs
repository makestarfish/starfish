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
}

impl Config {
  pub fn from_env() -> Self {
    envy::from_env::<Config>().expect("invalid environment variables")
  }
}
