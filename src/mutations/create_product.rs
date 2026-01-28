use crate::{
  context::RequestContext,
  entities::Product,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use starfish_stripe::types::CreateProductParams;
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  name: String,
  description: Option<String>,
) -> Result<Product, Failure> {
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
    &store_id,
    &user_id
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

  let mut create_product_params = CreateProductParams::new(&name);

  if let Some(description) = description.as_ref() {
    create_product_params = create_product_params.with_description(description);
  }

  let stripe_product = state
    .stripe
    .products
    .create(create_product_params)
    .await
    .map_err(|_| failure!())?;

  let product = sqlx::query_as!(
    Product,
    r#"
      insert into products (stripe_id, store_id, name, description)
      values ($1, $2, $3, $4)
      returning id, store_id, name, description, archived, created_at, modified_at
    "#,
    &stripe_product.id,
    &store_id,
    &name,
    description
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(product)
}
