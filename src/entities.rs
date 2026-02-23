use async_graphql::{Enum, InputObject, NewType, SimpleObject};
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

#[derive(
  NewType, sqlx::Type, Clone, PartialEq, Eq, Hash, Debug, Deserialize,
)]
#[sqlx(transparent)]
pub struct StoreId(pub Uuid);

#[derive(Enum, sqlx::Type, Clone, Debug, Copy, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum StoreStatus {
  Created,
  OnboardingStarted,
  Denied,
  Active,
}

#[derive(SimpleObject, FromRow, Clone)]
#[graphql(rename_fields = "snake_case", complex)]
pub struct Store {
  pub id: StoreId,
  pub slug: String,
  pub name: String,
  pub status: StoreStatus,
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

#[derive(
  NewType, sqlx::Type, Clone, PartialEq, Eq, Hash, Debug, Deserialize,
)]
#[sqlx(transparent)]
pub struct AccountId(pub Uuid);

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct Account {
  pub id: AccountId,
  pub stripe_id: String,
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

#[derive(
  NewType, sqlx::Type, Clone, PartialEq, Eq, Hash, Deserialize, Debug,
)]
#[sqlx(transparent)]
pub struct CustomerId(pub Uuid);

#[derive(SimpleObject, FromRow, Clone)]
#[graphql(rename_fields = "snake_case")]
pub struct Customer {
  pub id: CustomerId,
  pub store_id: StoreId,
  pub email: String,
  pub name: Option<String>,
  pub avatar_url: String,
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

#[derive(SimpleObject, FromRow, Clone, Deserialize, Debug)]
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

#[derive(Enum, sqlx::Type, Clone, Debug, Copy, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
  Open,
  Expired,
  Confirmed,
  Failed,
  Succeeded,
}

#[derive(
  NewType, sqlx::Type, Clone, PartialEq, Eq, Hash, Deserialize, Debug,
)]
#[sqlx(transparent)]
pub struct CheckoutSessionId(pub Uuid);

#[derive(SimpleObject, Clone)]
#[graphql(rename_fields = "snake_case", complex)]
pub struct CheckoutSession {
  pub id: CheckoutSessionId,
  pub store_id: StoreId,
  pub product_id: ProductId,
  pub customer_id: Option<CustomerId>,
  pub customer_email: Option<String>,
  pub client_secret: String,
  pub amount: i64,
  pub tax_amount: Option<i64>,
  pub discount_amount: i64,
  pub net_amount: i64,
  pub total_amount: i64,
  pub status: CheckoutSessionStatus,
  pub url: String,
  pub success_url: Option<String>,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CheckoutSessionEdge {
  pub cursor: CheckoutSessionId,
  pub node: CheckoutSession,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CheckoutSessionConnection {
  pub edges: Vec<CheckoutSessionEdge>,
  pub nodes: Vec<CheckoutSession>,
}

#[derive(NewType, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct CheckoutLinkId(pub Uuid);

#[derive(SimpleObject, Clone, FromRow)]
#[graphql(rename_fields = "snake_case")]
pub struct CheckoutLink {
  pub id: CheckoutLinkId,
  pub store_id: StoreId,
  pub client_secret: String,
  pub label: Option<String>,
  pub success_url: Option<String>,
  pub url: String,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct DeletedCheckoutLink {
  pub id: CheckoutLinkId,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CheckoutLinkEdge {
  pub cursor: CheckoutLinkId,
  pub node: CheckoutLink,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct CheckoutLinkConnection {
  pub edges: Vec<CheckoutLinkEdge>,
  pub nodes: Vec<CheckoutLink>,
}

#[derive(Enum, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum OrderStatus {
  Pending,
  Paid,
  Refunded,
  PartiallyRefunded,
}

#[derive(Enum, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum BillingReason {
  Purchase,
  SubscriptionCreation,
  SubscriptionRenewal,
  SubscriptionUpdate,
}

#[derive(
  NewType, sqlx::Type, Clone, PartialEq, Eq, Hash, Deserialize, Debug,
)]
#[sqlx(transparent)]
pub struct OrderId(pub Uuid);

#[derive(SimpleObject, Clone)]
#[graphql(rename_fields = "snake_case", complex)]
pub struct Order {
  pub id: OrderId,
  pub store_id: StoreId,
  pub customer_id: CustomerId,
  pub checkout_session_id: Option<CheckoutSessionId>,
  pub status: OrderStatus,
  pub subtotal_amount: i64,
  pub discount_amount: i64,
  pub net_amount: i64,
  pub tax_amount: i64,
  pub total_amount: i64,
  pub platform_fee_amount: i64,
  pub billing_reason: BillingReason,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct OrderEdge {
  pub cursor: OrderId,
  pub node: Order,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct OrderConnection {
  pub edges: Vec<OrderEdge>,
  pub nodes: Vec<Order>,
}

#[derive(NewType, sqlx::Type, Clone, Deserialize, Debug)]
#[sqlx(transparent)]
pub struct OrderItemId(pub Uuid);

#[derive(SimpleObject, Clone, Deserialize, Debug)]
#[graphql(rename_fields = "snake_case")]
pub struct OrderItem {
  pub id: OrderItemId,
  pub order_id: OrderId,
  pub product_price_id: PriceId,
  pub label: String,
  pub amount: i64,
  pub tax_amount: i64,
  pub created_at: DateTime<Utc>,
  pub modified_at: Option<DateTime<Utc>>,
}

#[derive(NewType, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct TransactionId(pub Uuid);

#[derive(SimpleObject, Clone)]
pub struct Transaction {
  pub id: TransactionId,
  pub account_id: AccountId,
  pub amount: i64,
  pub fee_amount: i64,
  pub net_amount: i64,
  pub created_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct TransactionEdge {
  pub cursor: TransactionId,
  pub node: Transaction,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct TransactionConnection {
  pub edges: Vec<TransactionEdge>,
  pub nodes: Vec<Transaction>,
}

#[derive(SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct Balance {
  pub amount: i64,
}
