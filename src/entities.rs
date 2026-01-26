use async_graphql::{ComplexObject, Context, NewType, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
  context::RequestContext, failure::Failure, queries::store_members,
  state::SharedState,
};

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

#[derive(NewType, Clone)]
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

#[derive(NewType, FromRow, Clone, Debug)]
pub struct StoreId(pub Uuid);

#[derive(SimpleObject, FromRow, Clone, Debug)]
#[graphql(rename_fields = "snake_case", complex)]
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

#[ComplexObject]
impl Store {
  async fn members(
    &self,
    context: &Context<'_>,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<StoreMemberConnection, Failure> {
    store_members::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      self,
      first,
      after,
      last,
      before,
    )
    .await
  }
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreEdge {
  pub cursor: StoreId,
  pub node: Store,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreConnection {
  pub edges: Vec<StoreEdge>,
  pub nodes: Vec<Store>,
}

#[derive(NewType, Clone)]
pub struct StoreMemberId(Uuid);

#[derive(SimpleObject, Clone)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreMember {
  pub id: StoreMemberId,
  pub user_id: UserId,
  pub store_id: StoreId,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreMemberEdge {
  pub cursor: StoreMemberId,
  pub node: StoreMember,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreMemberConnection {
  pub edges: Vec<StoreMemberEdge>,
  pub nodes: Vec<StoreMember>,
}
