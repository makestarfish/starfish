use crate::{
  context::RequestContext,
  entities::Product,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<Option<Product>, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let product = sqlx::query_as!(
    Product,
    r#"
      select 
        p.id, 
        p.store_id, 
        p.name, 
        p.description, 
        p.archived, 
        p.created_at, 
        p.modified_at
      from products p
      where 
        id = $1 and
        exists (
          select 1
          from store_members sm
          where sm.store_id = p.store_id and sm.user_id = $2
        )
    "#,
    &id,
    &user_id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(product)
}
