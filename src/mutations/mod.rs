use crate::{
  context::RequestContext,
  entities::{
    CheckoutLink, CheckoutSession, CreateProductPrice, Customer,
    DeletedCheckoutLink, DeletedCustomer, OneTimeToken, Product,
    RevokedSession, Store, StoreInvite, Tokens,
  },
  failure::Failure,
  state::SharedState,
};
use async_graphql::{Context, MaybeUndefined, Object};
use uuid::Uuid;

pub mod confirm_checkout_session;
pub mod create_checkout_link;
pub mod create_checkout_session;
pub mod create_customer;
pub mod create_product;
pub mod create_store;
pub mod create_store_invite;
pub mod delete_checkout_link;
pub mod delete_customer;
pub mod login_with_email;
pub mod refresh_session;
pub mod revoke_current_session;
pub mod revoke_other_sessions;
pub mod revoke_session;
pub mod revoke_store_invite;
pub mod send_login_code;
pub mod update_checkout_link;
pub mod update_checkout_session;
pub mod update_customer;
pub mod update_product;
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

  #[graphql(name = "create_store_invite")]
  async fn create_store_invite(
    &self,
    context: &Context<'_>,
    store_id: Uuid,
    email: String,
  ) -> Result<StoreInvite, Failure> {
    create_store_invite::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      store_id,
      email,
    )
    .await
  }

  #[graphql(name = "revoke_store_invite")]
  async fn revoke_store_invite(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<StoreInvite, Failure> {
    revoke_store_invite::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
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

  #[graphql(name = "create_product")]
  async fn create_product(
    &self,
    context: &Context<'_>,
    #[graphql(name = "store_id")] store_id: Uuid,
    name: String,
    description: Option<String>,
    #[graphql(validator(min_items = 1, max_items = 1))] prices: Vec<
      CreateProductPrice,
    >,
  ) -> Result<Product, Failure> {
    create_product::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      store_id,
      name,
      description,
      prices,
    )
    .await
  }

  #[graphql(name = "update_product")]
  async fn update_product(
    &self,
    context: &Context<'_>,
    id: Uuid,
    name: Option<String>,
    description: MaybeUndefined<String>,
    archived: Option<bool>,
  ) -> Result<Product, Failure> {
    update_product::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
      name,
      description,
      archived,
    )
    .await
  }

  #[graphql(name = "create_checkout_session")]
  async fn create_checkout_session(
    &self,
    context: &Context<'_>,
    #[graphql(validator(min_items = 1))] products: Vec<Uuid>,
    #[graphql(name = "customer_id")] customer_id: Option<Uuid>,
    #[graphql(name = "customer_email")] customer_email: Option<String>,
  ) -> Result<CheckoutSession, Failure> {
    create_checkout_session::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      products,
      customer_id,
      customer_email,
    )
    .await
  }

  // #[graphql(name = "update_checkout_session")]
  // async fn update_checkout_session(
  //  &self,
  //  context: &Context<'_>,
  //  id: Uuid,
  //  #[graphql(name = "product_id")] product_id: Option<Uuid>,
  // ) -> Result<CheckoutSession, Failure> {
  //  update_checkout_session::resolve(
  //    context.data_unchecked::<SharedState>(),
  //    context.data_unchecked::<RequestContext>(),
  //    id,
  //    product_id,
  //  )
  //  .await
  // }

  #[graphql(name = "confirm_checkout_session")]
  async fn confirm_checkout_session(
    &self,
    context: &Context<'_>,
    #[graphql(name = "client_secret")] client_secret: String,
    #[graphql(name = "confirmation_token_id")] confirmation_token_id: String,
    #[graphql(name = "customer_email")] customer_email: Option<String>,
  ) -> Result<CheckoutSession, Failure> {
    confirm_checkout_session::resolve(
      context.data_unchecked::<SharedState>(),
      client_secret,
      confirmation_token_id,
      customer_email,
    )
    .await
  }

  #[graphql(name = "create_checkout_link")]
  async fn create_checkout_link(
    &self,
    context: &Context<'_>,
    #[graphql(validator(min_items = 1))] products: Vec<Uuid>,
    label: Option<String>,
    #[graphql(name = "success_url")] success_url: Option<String>,
  ) -> Result<CheckoutLink, Failure> {
    create_checkout_link::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      products,
      label,
      success_url,
    )
    .await
  }

  #[graphql(name = "update_checkout_link")]
  async fn update_checkout_link(
    &self,
    context: &Context<'_>,
    id: Uuid,
    label: MaybeUndefined<String>,
    #[graphql(name = "success_url")] success_url: MaybeUndefined<String>,
  ) -> Result<CheckoutLink, Failure> {
    update_checkout_link::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
      label,
      success_url,
    )
    .await
  }

  #[graphql(name = "delete_checkout_link")]
  async fn delete_checkout_link(
    &self,
    context: &Context<'_>,
    id: Uuid,
  ) -> Result<DeletedCheckoutLink, Failure> {
    delete_checkout_link::resolve(
      context.data_unchecked::<SharedState>(),
      context.data_unchecked::<RequestContext>(),
      id,
    )
    .await
  }
}
