use async_graphql::{NewType, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

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

#[derive(NewType)]
pub struct UserId(Uuid);

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct User {
  pub id: UserId,
  pub email: String,
  pub name: Option<String>,
  pub avatar_url: Option<String>,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(NewType)]
pub struct SessionId(Uuid);

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct Session {
  pub id: SessionId,
  pub user_id: UserId,
  pub last_seen_at: DateTime<Utc>,
  pub is_current_session: bool,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct RevokedSession {
  pub id: SessionId,
}

#[derive(NewType, FromRow, Debug)]
pub struct StoreId(pub Uuid);

#[derive(SimpleObject, FromRow, Debug)]
#[graphql(rename_fields = "snake_case")]
pub struct Store {
  #[sqlx(flatten)]
  pub id: StoreId,
  pub slug: String,
  pub name: String,
  pub email: Option<String>,
  pub website: Option<String>,
  pub avatar_url: Option<String>,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreConnection {
  pub nodes: Vec<Store>,
}
