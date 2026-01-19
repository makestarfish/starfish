use chrono::{Duration, Utc};
use jsonwebtoken::{
  Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionClaims {
  #[serde(rename = "sub")]
  pub user_id: String,
  pub session_id: String,

  #[serde(rename = "exp")]
  pub expires_at: i64,
}

impl SessionClaims {
  pub fn new(user_id: impl ToString, session_id: impl ToString) -> Self {
    Self {
      user_id: user_id.to_string(),
      session_id: session_id.to_string(),
      expires_at: (Utc::now() + Duration::hours(1)).timestamp(),
    }
  }

  pub fn encode(&self, secret: &str) -> String {
    encode_claims(self, secret)
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
  #[serde(rename = "jti")]
  pub refresh_token_id: String,

  #[serde(rename = "exp")]
  pub expires_at: i64,
}

impl RefreshTokenClaims {
  pub fn new(refresh_token_id: impl ToString) -> Self {
    Self {
      refresh_token_id: refresh_token_id.to_string(),
      expires_at: (Utc::now() + Duration::days(30)).timestamp(),
    }
  }

  pub fn encode(&self, secret: &str) -> String {
    encode_claims(self, secret)
  }
}

pub fn encode_claims<T>(claims: &T, secret: &str) -> String
where
  T: Serialize,
{
  encode(
    &Header::default(),
    claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .unwrap()
}

pub fn decode_claims<T>(
  token: &str,
  secret: &str,
) -> jsonwebtoken::errors::Result<T>
where
  T: DeserializeOwned,
{
  decode(
    token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::new(Algorithm::HS256),
  )
  .map(|data| data.claims)
}
