use crate::{
  context::RequestContext,
  entities::DeletedCustomer,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<DeletedCustomer, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let customer = sqlx::query!(
    r#"
      select exists (
        select 1
        from customers c
        where c.id = $1 and deleted_at is null and exists (
          select 1
          from store_members sm
          where sm.store_id = c.store_id and sm.user_id = $2
        )
      )
    "#,
    &id,
    &user_id,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !customer.exists.unwrap_or_default() {
    bail!(
      FailureReason::NOT_FOUND,
      "The customer '{id}' could not be found"
    )
  }

  let deleted_customer = sqlx::query_as!(
    DeletedCustomer,
    r#"
      update customers
      set deleted_at = now()
      where id = $1
      returning id
    "#,
    &id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(deleted_customer)
}
