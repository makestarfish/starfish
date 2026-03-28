use crate::{
  context::RequestContext,
  entities::{Store, StoreStatus},
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  slug: String,
  name: String,
  email: Option<String>,
  website: Option<String>,
  avatar_url: Option<String>,
) -> Result<Store, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store_with_same_slug = sqlx::query!(
    r#"
      select exists (
        select 1
        from stores
        where slug = $1
      )
    "#,
    &slug
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if store_with_same_slug.exists.unwrap_or_default() {
    bail!(
      FailureReason::CONFLICT,
      "The slug '{slug}' is already in use"
    )
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let store = sqlx::query_as!(
    Store,
    r#"
      insert into stores (slug, name, email, website, avatar_url)
      values ($1, $2, $3, $4, $5)
      returning 
        id, 
        slug, 
        name, 
        status as "status: StoreStatus",
        email, 
        website, 
        avatar_url, 
        created_at, 
        modified_at
    "#,
    &slug,
    &name,
    email,
    website,
    avatar_url
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      insert into store_members (store_id, user_id)
      values ($1, $2)
    "#,
    &store.id.0,
    &user_id
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(store)
}
