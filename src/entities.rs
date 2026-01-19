use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct OneTimeToken {
  pub id: String,
  pub email: String,
  pub expires_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct Tokens {
  pub access_token: String,
  pub refresh_token: String,
  pub access_token_expires_at: i64,
  pub refresh_token_expires_at: i64,
}
