use uuid::Uuid;

use crate::{
  context::RequestContext,
  entities::CheckoutLink,
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::create_client_secret,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  products: Vec<Uuid>,
  label: Option<String>,
  success_url: Option<String>,
) -> Result<CheckoutLink, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let product_data = sqlx::query!(
    r#"
      select p.id, p.store_id
      from products p
      where 
        p.id = any($1) and
        exists (
          select 1
          from store_members sm
          where sm.store_id = p.store_id and sm.user_id = $2
        )
    "#,
    &products,
    &user_id
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  if product_data.len() != products.len() {
    bail!(
      FailureReason::NOT_FOUND,
      "The product '{}' could not be found",
      products
        .iter()
        .find(|&&id| product_data.iter().all(|d| d.id != id))
        .unwrap()
    )
  }

  let product = product_data
    .iter()
    .find(|data| &data.id == products.first().unwrap())
    .unwrap();

  if let Some(product_from_other_store) = product_data
    .iter()
    .find(|&data| data.store_id != product.store_id)
  {
    bail!(
      FailureReason::CONFLICT,
      "The products '{}' and '{}' are from different stores",
      product.id,
      product_from_other_store.id
    )
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let client_secret = create_client_secret("starfish_cl_");

  let checkout_link = sqlx::query_as!(
    CheckoutLink,
    r#"
      insert into checkout_links(store_id, client_secret, label, success_url)
      values ($1, $2, $3, $4)
      returning
        id,
        store_id,
        client_secret,
        label,
        success_url,
        rtrim($5, '/') || '/' || client_secret as "url!",
        created_at,
        modified_at
    "#,
    &product.store_id,
    &client_secret,
    label,
    success_url,
    &state.config.website_base_url
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      insert into checkout_link_products (checkout_link_id, product_id)
      select $1, products
      from unnest($2::uuid[]) as products
    "#,
    &checkout_link.id.0,
    &products,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(checkout_link)
}
