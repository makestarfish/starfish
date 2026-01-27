use crate::{
  context::RequestContext,
  entities::{
    Customer, DeletedCustomer, OneTimeToken, RevokedSession, Store, Tokens,
  },
  failure::Failure,
  state::SharedState,
};
use async_graphql::{Context, MaybeUndefined, Object};
use uuid::Uuid;

pub mod create_customer;
pub mod create_store;
pub mod delete_customer;
pub mod login_with_email;
pub mod refresh_session;
pub mod revoke_current_session;
pub mod revoke_other_sessions;
pub mod revoke_session;
pub mod send_login_code;
pub mod update_customer;
pub mod update_store;

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

  #[graphql(name = "revoke_current_session")]
  async fn revoke_current_session(
    &self,
    context: &Context<'_>,
  ) -> Result<RevokedSession, Failure> {
    revoke_current_session::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
    )
    .await
  }

  #[graphql(name = "revoke_session")]
  async fn revoke_session(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<RevokedSession, Failure> {
    revoke_session::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }

  #[graphql(name = "revoke_other_sessions")]
  async fn revoke_other_sessions(
    &self,
    context: &Context<'_>,
  ) -> Result<bool, Failure> {
    revoke_other_sessions::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
    )
    .await
  }

  #[graphql(name = "create_store")]
  async fn create_store(
    &self,
    context: &Context<'_>,
    slug: String,
    name: String,
    email: Option<String>,
    website: Option<String>,
    #[graphql(name = "avatar_url")] avatar_url: Option<String>,
  ) -> Result<Store, Failure> {
    create_store::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      slug,
      name,
      email,
      website,
      avatar_url,
    )
    .await
  }

  #[graphql(name = "update_store")]
  async fn update_store(
    &self,
    context: &Context<'_>,
    slug: String,
    name: Option<String>,
    email: MaybeUndefined<String>,
    website: MaybeUndefined<String>,
    #[graphql(name = "avatar_url")] avatar_url: MaybeUndefined<String>,
  ) -> Result<Store, Failure> {
    update_store::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      slug,
      name,
      email,
      website,
      avatar_url,
    )
    .await
  }

  #[graphql(name = "create_customer")]
  async fn create_customer(
    &self,
    context: &Context<'_>,
    #[graphql(name = "store_id")] store_id: Uuid,
    email: String,
    name: Option<String>,
  ) -> Result<Customer, Failure> {
    create_customer::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      store_id,
      email,
      name,
    )
    .await
  }

  #[graphql(name = "update_customer")]
  async fn update_customer(
    &self,
    context: &Context<'_>,
    id: Uuid,
    email: Option<String>,
    name: MaybeUndefined<String>,
  ) -> Result<Customer, Failure> {
    update_customer::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
      email,
      name,
    )
    .await
  }

  #[graphql(name = "delete_customer")]
  async fn delete_customer(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<DeletedCustomer, Failure> {
    delete_customer::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }
}
