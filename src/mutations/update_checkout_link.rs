use crate::{
  context::RequestContext,
  entities::CheckoutLink,
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
        rtrim($3, '/') || '/' || cl.client_secret as "url!",
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

  let mut query_builder = QueryBuilder::new("update checkout_links ");
  let mut separated = query_builder.separated(", ");

  if let Some(label) = label.as_opt_ref() {
    separated.push("label = ");
    separated.push_bind_unseparated(label);
  }

  if let Some(success_url) = success_url.as_opt_ref() {
    separated.push("success_url = ");
    separated.push_bind_unseparated(success_url);
  }

  separated.push("modified_at = now()");

  query_builder.push(" where id = ");
  query_builder.push_bind(id);

  query_builder.push(" returning id, store_id, client_secret, label, success_url, created_at, modified_at, rtrim(");
  query_builder.push_bind(&state.config.website_base_url);
  query_builder.push(", '/') || '/' || client_secret as url");

  let updated_checkout_link = query_builder
    .build_query_as::<CheckoutLink>()
    .fetch_one(&state.db)
    .await
    .map_err(|_| failure!())?;

  Ok(updated_checkout_link)
}
