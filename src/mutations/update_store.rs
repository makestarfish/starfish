use crate::{
  context::RequestContext,
  entities::{Store, StoreId},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use async_graphql::MaybeUndefined;
use sqlx::QueryBuilder;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  slug: String,
  name: Option<String>,
  email: MaybeUndefined<String>,
  website: MaybeUndefined<String>,
  avatar_url: MaybeUndefined<String>,
) -> Result<Store, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store = sqlx::query!(
    r#"
      select 
        id,
        slug,
        name,
        email,
        website,
        avatar_url,
        created_at,
        modified_at,
        exists (
          select 1
          from store_members sm
          where sm.store_id = s.id and sm.user_id = $2
        ) as member_exists
      from stores s
      where s.slug = $1
    "#,
    &slug,
    &user_id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The store '{slug}' could not be found"
    )
  })?;

  if !store.member_exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    );
  }

  if name.is_none()
    && email.is_undefined()
    && website.is_undefined()
    && avatar_url.is_undefined()
  {
    return Ok(Store {
      id: StoreId(store.id),
      slug: store.slug,
      name: store.name,
      email: store.email,
      website: store.website,
      avatar_url: store.avatar_url,
      created_at: store.created_at,
      modified_at: store.modified_at,
    });
  }

  let mut query_builder = QueryBuilder::new("update stores set ");
  let mut separated = query_builder.separated(", ");

  if let Some(name) = name {
    separated.push("name = ");
    separated.push_bind_unseparated(name);
  }

  if let Some(email) = email.as_opt_ref() {
    separated.push("email = ");
    separated.push_bind_unseparated(email);
  }

  if let Some(website) = website.as_opt_ref() {
    separated.push("website = ");
    separated.push_bind_unseparated(website);
  }

  if let Some(avatar_url) = avatar_url.as_opt_ref() {
    separated.push("avatar_url = ");
    separated.push_bind_unseparated(avatar_url);
  }

  separated.push("modified_at = now()");

  query_builder.push(" where id = ");
  query_builder.push_bind(store.id);

  query_builder
    .push(" returning id, slug, name, email, website, avatar_url, created_at, modified_at");

  let updated_store = query_builder
    .build_query_as::<Store>()
    .fetch_one(&state.db)
    .await
    .map_err(|_| failure!())?;

  Ok(updated_store)
}
