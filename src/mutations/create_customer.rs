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

  let customer = sqlx::query_as!(
    Customer,
    r#"
      insert into customers (store_id, email, name)
      values ($1, $2, $3)
      returning id, store_id, email, name, created_at, modified_at
    "#,
    &store_id,
    &email,
    name
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(customer)
}
