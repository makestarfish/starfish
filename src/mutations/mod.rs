use crate::{
  entities::{OneTimeToken, Tokens},
  failure::Failure,
  state::SharedState,
};
use async_graphql::{Context, Object};
use uuid::Uuid;

pub mod login_with_email;
pub mod request_auth_email;

pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(name = "request_auth_email")]
  async fn request_auth_email(
    &self,
    context: &Context<'_>,
    #[graphql(validator(email))] email: String,
  ) -> Result<OneTimeToken, Failure> {
    request_auth_email::resolve(
      context.data_unchecked::<SharedState>().clone(),
      email,
    )
    .await
  }

  #[graphql(name = "login_with_email")]
  async fn login_with_email(
    &self,
    context: &Context<'_>,
    #[graphql(name = "one_time_token_id")] one_time_token_id: Uuid,
    #[graphql(validator(email))] email: String,
    code: String,
  ) -> Result<Tokens, Failure> {
    login_with_email::resolve(
      context.data_unchecked::<SharedState>().clone(),
      one_time_token_id,
      email,
      code,
    )
    .await
  }
}
