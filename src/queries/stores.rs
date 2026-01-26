use crate::{
  context::RequestContext,
  entities::{Store, StoreConnection, StoreEdge},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  _first: Option<i64>,
  _after: Option<Uuid>,
  _last: Option<i64>,
  _before: Option<Uuid>,
) -> Result<StoreConnection, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let stores = sqlx::query_as!(
    Store,
    r#"
      select 
        s.id, 
        s.slug, 
        s.name, 
        s.email, 
        s.website, 
        s.avatar_url, 
        s.created_at, 
        s.modified_at
      from stores s
      where 
        exists (
          select 1
          from store_members sm
          where s.id = sm.store_id and sm.user_id = $1
        )
    "#,
    &user_id,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(StoreConnection {
    edges: stores
      .iter()
      .map(|store| StoreEdge {
        cursor: store.id.to_owned(),
        node: store.clone(),
      })
      .collect(),
    nodes: stores,
  })
}
