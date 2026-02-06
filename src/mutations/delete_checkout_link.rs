use uuid::Uuid;

use crate::{
  context::RequestContext,
  entities::DeletedCheckoutLink,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<DeletedCheckoutLink, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let checkout_link = sqlx::query!(
    r#"
      select exists (
        select 1
        from checkout_links cl
        where 
          cl.id = $1 and 
          cl.deleted_at is null and 
          exists (
            select 1
            from store_members sm
            where sm.store_id = cl.store_id and sm.user_id = $2
          )
      )
    "#,
    &id,
    &user_id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !checkout_link.exists.unwrap_or_default() {
    bail!(
      FailureReason::NOT_FOUND,
      "The checkout link '{id}' could not be found"
    )
  }

  let deleted_checkout_link = sqlx::query_as!(
    DeletedCheckoutLink,
    r#"
      update checkout_links
      set deleted_at = now()
      where id = $1
      returning id
    "#,
    &id,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(deleted_checkout_link)
}
