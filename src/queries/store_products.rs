use crate::{
  context::RequestContext,
  entities::{Product, ProductConnection, ProductEdge, Store},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store: &Store,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
  archived: Option<bool>,
) -> Result<ProductConnection, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store_member = sqlx::query!(
    r#"
      select exists (
        select 1
        from store_members
        where store_id = $1 and user_id = $2
      )
    "#,
    &store.id.0,
    &user_id,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !store_member.exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  let products = sqlx::query_as!(
    Product,
    r#"
      select id, store_id, name, description, archived, created_at, modified_at
      from products
      where 
        store_id = $1 and 
        archived = $2 and
        (id < $3 or $3 is null) and
        (id > $4 or $4 is null)
      order by id desc
      limit 20
    "#,
    &store.id.0,
    archived.unwrap_or(false),
    after,
    before,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(ProductConnection {
    edges: products
      .iter()
      .map(|product| ProductEdge {
        cursor: product.id.to_owned(),
        node: product.clone(),
      })
      .collect(),
    nodes: products,
  })
}
