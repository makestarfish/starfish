use crate::{
  context::RequestContext,
  entities::Customer,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<Customer, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(crate::failure::FailureReason::UNAUTHORIZED))?;

  let customer = sqlx::query_as!(
    Customer,
    r#"
      select 
        c.id, 
        c.store_id, 
        c.email, 
        c.name, 
        c.avatar_url, 
        c.created_at, 
        c.modified_at
      from customers c
      where 
        c.id = $1 and
        c.deleted_at is null and
        exists (
          select 1
          from store_members sm
          where sm.store_id = c.store_id and sm.user_id = $2
        )
    "#,
    &id,
    &user_id
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

  Ok(customer)
}
