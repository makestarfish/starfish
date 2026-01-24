use crate::{entities::Store, failure::Failure, state::SharedState};

pub async fn resolve(
  state: &SharedState,
  slug: String,
) -> Result<Option<Store>, Failure> {
  sqlx::query_as!(
    Store,
    r#"
      select id, slug, name, email, website, avatar_url, created_at, modified_at
      from stores
      where slug = $1
    "#,
    &slug
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())
}
