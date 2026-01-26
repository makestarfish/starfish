use crate::{
  context::RequestContext,
  entities::{Session, Store, StoreConnection, User},
  failure::Failure,
  state::SharedState,
};
use async_graphql::{Context, Object};
use uuid::Uuid;

pub mod session;
pub mod sessions;
pub mod store;
pub mod store_members;
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
}
