use chrono::{Duration, Utc};
use jsonwebtoken::{
  Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionClaims {
  #[serde(rename = "sub")]
  pub user_id: Uuid,
  pub session_id: Uuid,

  #[serde(rename = "exp")]
  pub expires_at: i64,
}

impl SessionClaims {
  pub fn new(user_id: Uuid, session_id: Uuid) -> Self {
    Self {
      user_id,
      session_id,
      expires_at: (Utc::now() + Duration::hours(1)).timestamp(),
    }
  }

  pub fn from_token(
    token: &str,
    signing_secret: &str,
  ) -> jsonwebtoken::errors::Result<Self> {
    decode_claims(token, signing_secret)
  }

  pub fn encode(&self, secret: &str) -> String {
    encode_claims(self, secret)
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
  #[serde(rename = "jti")]
  pub refresh_token_id: Uuid,

  #[serde(rename = "exp")]
  pub expires_at: i64,
}

impl RefreshTokenClaims {
  pub fn new(refresh_token_id: Uuid) -> Self {
    Self {
      refresh_token_id,
      expires_at: (Utc::now() + Duration::days(30)).timestamp(),
    }
  }

  pub fn from_token(
    token: &str,
    signing_secret: &str,
  ) -> jsonwebtoken::errors::Result<Self> {
    decode_claims(token, signing_secret)
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
