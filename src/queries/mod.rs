use crate::{
  context::RequestContext,
  dataloader::{PriceLoader, ProductLoader, StandardLoader},
  entities::{
    Account, Balance, CheckoutLink, CheckoutLinkConnection, CheckoutSession,
    CheckoutSessionConnection, Customer, CustomerConnection, Order,
    OrderConnection, OrderItem, Price, Product, ProductConnection, Session,
    Store, StoreConnection, StoreInvite, StoreInviteConnection,
    StoreMemberConnection, Transaction, TransactionConnection, User,
  },
  failure::Failure,
  state::SharedState,
};
use async_graphql::{ComplexObject, Context, Object, dataloader::DataLoader};
use uuid::Uuid;

pub mod account;
pub mod balance;
pub mod checkout_link;
pub mod checkout_links;
pub mod checkout_session;
pub mod checkout_sessions;
pub mod customer;
pub mod order;
pub mod orders;
pub mod product;
pub mod session;
pub mod sessions;
pub mod store;
pub mod store_customers;
pub mod store_invite;
pub mod store_invites;
pub mod store_members;
pub mod store_products;
pub mod stores;
mod transaction;
mod transactions;
pub mod viewer;

pub struct Query;

#[Object]
impl Query {
  pub async fn viewer(&self, context: &Context<'_>) -> Result<User, Failure> {
    viewer::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
    )
    .await
  }

  pub async fn sessions(
    &self,
    context: &Context<'_>,
  ) -> Result<Vec<Session>, Failure> {
    sessions::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
    )
    .await
  }

  pub async fn session(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<Session, Failure> {
    session::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  pub async fn stores(
    &self,
    context: &Context<'_>,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<StoreConnection, Failure> {
    stores::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      first,
      after,
      last,
      before,
    )
    .await
  }

  async fn store(
    &self,
    context: &Context<'_>,
    slug: String,
  ) -> Result<Option<Store>, Failure> {
    store::resolve(context.data_unchecked::<SharedState>(), slug).await
  }

  async fn customer(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<Customer, Failure> {
    customer::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  async fn product(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<Product, Failure> {
    product::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  #[graphql(name = "store_invite")]
  async fn store_invite(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<Option<StoreInvite>, Failure> {
    store_invite::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  #[graphql(name = "checkout_sessions")]
  async fn checkout_sessions(
    &self,
    context: &Context<'_>,
    #[graphql(name = "store_id")] store_id: Uuid,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<CheckoutSessionConnection, Failure> {
    checkout_sessions::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      store_id,
      first,
      after,
      last,
      before,
    )
    .await
  }

  #[graphql(name = "checkout_session")]
  async fn checkout_session(
    &self,
    context: &Context<'_>,
    #[graphql(name = "client_secret")] client_secret: String,
  ) -> Result<CheckoutSession, Failure> {
    checkout_session::resolve(
      context.data_unchecked::<SharedState>(),
      client_secret,
    )
    .await
  }

  #[graphql(name = "checkout_links")]
  async fn checkout_links(
    &self,
    context: &Context<'_>,
    #[graphql(name = "store_id")] store_id: Uuid,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<CheckoutLinkConnection, Failure> {
    checkout_links::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      store_id,
      first,
      after,
      last,
      before,
    )
    .await
  }

  #[graphql(name = "checkout_link")]
  async fn checkout_link(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<CheckoutLink, Failure> {
    checkout_link::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  async fn orders(
    &self,
    context: &Context<'_>,
    #[graphql(name = "store_id")] store_id: Uuid,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<OrderConnection, Failure> {
    orders::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      store_id,
      first,
      after,
      last,
      before,
    )
    .await
  }

  async fn order(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<Order, Failure> {
    order::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  async fn transactions(
    &self,
    context: &Context<'_>,
    #[graphql(name = "account_id")] account_id: Uuid,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<TransactionConnection, Failure> {
    transactions::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      account_id,
      first,
      after,
      last,
      before,
    )
    .await
  }

  async fn transaction(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<Transaction, Failure> {
    transaction::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  async fn balance(
    &self,
    context: &Context<'_>,
    #[graphql(name = "account_id")] account_id: Uuid,
  ) -> Result<Balance, Failure> {
    balance::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      account_id,
    )
    .await
  }
}

#[ComplexObject]
impl Store {
  async fn account(&self, context: &Context<'_>) -> Result<Account, Failure> {
    account::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      self.id.0,
    )
    .await
  }

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

  async fn customers(
    &self,
    context: &Context<'_>,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
  ) -> Result<CustomerConnection, Failure> {
    store_customers::resolve(
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

  async fn products(
    &self,
    context: &Context<'_>,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
    archived: Option<bool>,
  ) -> Result<ProductConnection, Failure> {
    store_products::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      self,
      first,
      after,
      last,
      before,
      archived,
    )
    .await
  }

  async fn invites(
    &self,
    context: &Context<'_>,
    first: Option<i64>,
    after: Option<Uuid>,
    last: Option<i64>,
    before: Option<Uuid>,
    revoked: Option<bool>,
  ) -> Result<StoreInviteConnection, Failure> {
    store_invites::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      self,
      first,
      after,
      last,
      before,
      revoked,
    )
    .await
  }
}

#[ComplexObject]
impl Product {
  async fn prices(&self, context: &Context<'_>) -> Result<Vec<Price>, Failure> {
    context
      .data_unchecked::<DataLoader<PriceLoader>>()
      .load_one(self.id.to_owned())
      .await
      .map(|prices| prices.unwrap())
  }
}

#[ComplexObject]
impl CheckoutSession {
  async fn store(&self, context: &Context<'_>) -> Result<Store, Failure> {
    context
      .data_unchecked::<DataLoader<StandardLoader>>()
      .load_one(self.store_id.to_owned())
      .await
      .map(|store| store.unwrap())
  }

  async fn product(&self, context: &Context<'_>) -> Result<Product, Failure> {
    context
      .data_unchecked::<DataLoader<StandardLoader>>()
      .load_one(self.product_id.to_owned())
      .await
      .map(|store| store.unwrap())
  }

  async fn products(
    &self,
    context: &Context<'_>,
  ) -> Result<Vec<Product>, Failure> {
    context
      .data_unchecked::<DataLoader<ProductLoader>>()
      .load_one(self.id.to_owned())
      .await
      .map(|products| products.unwrap())
  }
}

#[ComplexObject]
impl Order {
  async fn customer(&self, context: &Context<'_>) -> Result<Customer, Failure> {
    context
      .data_unchecked::<DataLoader<StandardLoader>>()
      .load_one(self.customer_id.to_owned())
      .await
      .map(|customer| customer.unwrap())
  }

  #[graphql(name = "checkout_session")]
  async fn checkout_session(
    &self,
    context: &Context<'_>,
  ) -> Result<Option<CheckoutSession>, Failure> {
    match self.checkout_session_id.as_ref() {
      Some(checkout_session_id) => {
        context
          .data_unchecked::<DataLoader<StandardLoader>>()
          .load_one(checkout_session_id.to_owned())
          .await
      }
      _ => Ok(None),
    }
  }

  async fn items(
    &self,
    context: &Context<'_>,
  ) -> Result<Vec<OrderItem>, Failure> {
    context
      .data_unchecked::<DataLoader<StandardLoader>>()
      .load_one(self.id.to_owned())
      .await
      .map(|items| items.unwrap())
  }
}
