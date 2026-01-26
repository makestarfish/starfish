use crate::{
  context::RequestContext,
  entities::Customer,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use async_graphql::MaybeUndefined;
use sqlx::QueryBuilder;
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
  email: Option<String>,
  name: MaybeUndefined<String>,
) -> Result<Customer, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let customer = sqlx::query_as!(
    Customer,
    r#"
      select c.id, c.store_id, c.name, c.email, c.created_at, c.modified_at
      from customers c
      where c.id = $1 and exists (
        select 1
        from store_members sm
        where sm.store_id = c.store_id and sm.user_id = $2
      )
    "#,
    &id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The customer '{id}' could not be found"
    )
  })?;

  if email.is_none() && name.is_undefined() {
    return Ok(customer);
  }

  let mut query_builder = QueryBuilder::new("update customers set ");
  let mut separated = query_builder.separated(", ");

  if let Some(email) = email {
    separated.push("email = ");
    separated.push_bind_unseparated(email);
  }

  if let Some(name) = name.as_opt_ref() {
    separated.push("name = ");
    separated.push_bind_unseparated(name);
  }

  separated.push("modified_at = now()");

  query_builder.push(" where id = ");
  query_builder.push_bind(id);

  query_builder
    .push(" returning id, store_id, email, name, created_at, modified_at");

  let updated_customer = query_builder
    .build_query_as::<Customer>()
    .fetch_one(&state.db)
    .await
    .map_err(|_| failure!())?;

  Ok(updated_customer)
}
