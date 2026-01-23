use crate::{
  entities::{OneTimeToken, Tokens},
  failure::Failure,
  state::SharedState,
};
use async_graphql::{Context, Object};
use uuid::Uuid;

pub mod login_with_email;
pub mod refresh_session;
pub mod send_login_code;

pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(name = "send_login_code")]
  async fn send_login_code(
    &self,
    context: &Context<'_>,
    #[graphql(validator(email))] email: String,
  ) -> Result<OneTimeToken, Failure> {
    send_login_code::resolve(
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

  #[graphql(name = "refresh_session")]
  async fn refresh_session(
    &self,
    context: &Context<'_>,
    #[graphql(name = "refresh_token")] refresh_token: String,
  ) -> Result<Tokens, Failure> {
    refresh_session::resolve(
      context.data_unchecked::<SharedState>(),
      refresh_token,
    )
    .await
  }
}
