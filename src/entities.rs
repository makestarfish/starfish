use async_graphql::{InputObject, NewType, SimpleObject};
use chrono::{DateTime, Utc};
use serde::Deserialize;
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

#[derive(NewType, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct StoreId(pub Uuid);

#[derive(SimpleObject, FromRow, Clone)]
#[graphql(rename_fields = "snake_case", complex)]
pub struct Store {
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

#[derive(NewType, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct CustomerId(Uuid);

#[derive(SimpleObject, FromRow, Clone)]
#[graphql(rename_fields = "snake_case")]
pub struct Customer {
  pub id: CustomerId,
  pub store_id: StoreId,
  pub email: String,
  pub name: Option<String>,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct DeletedCustomer {
  pub id: CustomerId,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CustomerEdge {
  pub cursor: CustomerId,
  pub node: Customer,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CustomerConnection {
  pub edges: Vec<CustomerEdge>,
  pub nodes: Vec<Customer>,
}

#[derive(
  NewType, sqlx::Type, Deserialize, Clone, PartialEq, Eq, Hash, Debug,
)]
#[sqlx(transparent)]
pub struct ProductId(pub Uuid);

#[derive(SimpleObject, FromRow, Clone)]
#[graphql(rename_fields = "snake_case", complex)]
pub struct Product {
  pub id: ProductId,
  pub store_id: StoreId,
  pub name: String,
  pub description: Option<String>,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(InputObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CreateProductPrice {
  pub amount: i64,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct ProductEdge {
  pub cursor: ProductId,
  pub node: Product,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct ProductConnection {
  pub edges: Vec<ProductEdge>,
  pub nodes: Vec<Product>,
}

#[derive(
  NewType, sqlx::Type, Clone, PartialEq, Eq, Hash, Deserialize, Debug,
)]
#[sqlx(transparent)]
pub struct PriceId(pub Uuid);

#[derive(SimpleObject, Deserialize, Clone, Debug)]
#[graphql(rename_fields = "snake_case")]
pub struct Price {
  pub id: PriceId,
  pub product_id: ProductId,
  pub amount: i64,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(NewType, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct StoreInviteId(pub Uuid);

#[derive(SimpleObject, Clone)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreInvite {
  pub id: StoreInviteId,
  pub store_id: StoreId,
  pub email: String,
  pub accepted_at: Option<DateTime<Utc>>,
  pub revoked_at: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreInviteEdge {
  pub cursor: StoreInviteId,
  pub node: StoreInvite,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct StoreInviteConnection {
  pub edges: Vec<StoreInviteEdge>,
  pub nodes: Vec<StoreInvite>,
}
