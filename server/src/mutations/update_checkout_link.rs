use crate::{
  context::RequestContext,
  entities::CheckoutLink,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use async_graphql::MaybeUndefined;
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
  label: MaybeUndefined<String>,
  success_url: MaybeUndefined<String>,
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
        rtrim($3, '/') || '/links/' || cl.client_secret as "url!",
        cl.created_at, 
        cl.modified_at
      from checkout_links cl
      where 
        id = $1 and
        exists (
          select 1
          from store_members sm
          where sm.store_id = cl.store_id and sm.user_id = $2
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

  if label.is_undefined() && success_url.is_undefined() {
    return Ok(checkout_link);
  }

  let updated_checkout_link = sqlx::query_as!(
    CheckoutLink,
    r#"
      update checkout_links 
      set label = $2, success_url = $3, modified_at = now()
      where id = $1
      returning
        id, 
        store_id, 
        client_secret, 
        label, 
        success_url, 
        rtrim($4, '/') || '/links/' || client_secret as "url!",
        created_at, 
        modified_at
    "#,
    &id,
    label.as_opt_ref().unwrap_or(checkout_link.label.as_ref()),
    success_url
      .as_opt_ref()
      .unwrap_or(checkout_link.success_url.as_ref()),
    &state.config.website_base_url,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(updated_checkout_link)
}
