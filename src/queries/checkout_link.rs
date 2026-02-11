use crate::{
  context::RequestContext,
  entities::CheckoutLink,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<CheckoutLink, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let checkout_link = sqlx::query_as!(
    CheckoutLink,
    r#"
      select 
        cl.id, 
        cl.store_id, 
        cl.client_secret, 
        cl.label, 
        cl.success_url, 
        rtrim($3, '/') || '/links' || cl.client_secret as "url!",
        cl.created_at, 
        cl.modified_at
      from checkout_links cl
      where 
        cl.id = $1 and 
        exists (
          select 1
          from store_members sm
          where sm.store_id = cl.store_id and user_id = $2
        )
    "#,
    &id,
    &user_id,
    &state.config.website_base_url,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The checkout link '{id}' could not be found"
    )
  })?;

  Ok(checkout_link)
}
