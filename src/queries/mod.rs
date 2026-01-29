use crate::{
  context::RequestContext,
  dataloader::DataLoader as StarfishLoader,
  entities::{
    Customer, CustomerConnection, Price, Product, ProductConnection, Session,
    Store, StoreConnection, StoreInvite, StoreInviteConnection,
    StoreMemberConnection, User,
  },
  failure::Failure,
  state::SharedState,
};
use async_graphql::{ComplexObject, Context, Object, dataloader::DataLoader};
use uuid::Uuid;

pub mod customer;
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
  ) -> Result<Option<Session>, Failure> {
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
  ) -> Result<Option<Customer>, Failure> {
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
  ) -> Result<Option<Product>, Failure> {
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
      .data_unchecked::<DataLoader<StarfishLoader>>()
      .load_one(self.id.to_owned())
      .await
      .map(|prices| prices.unwrap())
  }
}
