use crate::{
  context::RequestContext,
  entities::Product,
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
  name: Option<String>,
  description: MaybeUndefined<String>,
  archived: Option<bool>,
) -> Result<Product, Failure> {
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
        p.id = $1 and
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
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The product '{id}' could not be found"
    )
  })?;

  if name.is_none() && description.is_undefined() && archived.is_none() {
    return Ok(product);
  }

  let mut query_builder = QueryBuilder::new("update products set ");
  let mut separated = query_builder.separated(", ");

  if let Some(name) = name {
    separated.push("name = ");
    separated.push_bind_unseparated(name);
  }

  if let Some(description) = description.as_opt_ref() {
    separated.push("description = ");
    separated.push_bind_unseparated(description);
  }

  if let Some(archived) = archived {
    separated.push("archived = ");
    separated.push_bind_unseparated(archived);
  }

  separated.push("modified_at = now()");

  query_builder.push(" where id = ");
  query_builder.push_bind(id);

  query_builder
    .push(" returning id, store_id, name, description, archived, created_at, modified_at");

  let updated_product = query_builder
    .build_query_as::<Product>()
    .fetch_one(&state.db)
    .await
    .map_err(|_| failure!())?;

  Ok(updated_product)
}
