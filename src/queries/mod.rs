use crate::{
  context::RequestContext,
  entities::{Session, User},
  failure::Failure,
  state::SharedState,
};
use async_graphql::{Context, Object};
use uuid::Uuid;

pub mod session;
pub mod sessions;
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
}
