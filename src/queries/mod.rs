use crate::{
  context::RequestContext, entities::User, failure::Failure, state::SharedState,
};
use async_graphql::{Context, Object};

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

  async fn message(&self) -> String {
    String::from("hello")
  }
}
