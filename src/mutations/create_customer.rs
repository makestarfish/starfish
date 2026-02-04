use starfish_stripe::types::CreateCustomerParams;
use uuid::Uuid;

use crate::{
  context::RequestContext,
  entities::Customer,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  email: String,
  name: Option<String>,
) -> Result<Customer, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store_member = sqlx::query!(
    r#"
      select exists (
        select 1
        from store_members
        where store_id = $1 and user_id = $2
      )
    "#,
    &store_id,
    &user_id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !store_member.exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  let customer_with_same_email = sqlx::query!(
    r#"
      select exists (
        select 1
        from customers
        where store_id = $1 and email = $2 and deleted_at is null
      )
    "#,
    &store_id,
    &email
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if customer_with_same_email.exists.unwrap_or_default() {
    bail!(
      FailureReason::CONFLICT,
      "There is a customer with the same email"
    )
  }

  let mut create_customer_params =
    CreateCustomerParams::new().with_email(&email);

  if let Some(name) = name.as_ref() {
    create_customer_params = create_customer_params.with_name(name);
  }

  let stripe_customer = state
    .stripe
    .customers
    .create(create_customer_params)
    .await
    .map_err(|_| failure!())?;

  let customer = sqlx::query_as!(
    Customer,
    r#"
      insert into customers (stripe_id, store_id, email, name, avatar_url)
      values ($1, $2, $3, $4, $5)
      returning id, store_id, email, name, avatar_url, created_at, modified_at
    "#,
    &stripe_customer.id,
    &store_id,
    &email,
    name,
    format!(
      "https://www.gravatar.com/avatar/{:x}?d=404",
      md5::compute(email.as_bytes())
    )
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(customer)
}
