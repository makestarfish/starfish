use crate::{
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::create_client_secret,
};
use serde::Deserialize;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Product {
  pub id: Uuid,
  pub price_amount: i64,
}

pub async fn resolve(
  state: &SharedState,
  client_secret: String,
) -> Result<CheckoutSession, Failure> {
  let checkout_link = sqlx::query!(
    r#"
      select 
        cl.store_id,
        cl.success_url,
        jsonb_agg(
          jsonb_build_object(
            'id', p.product_id,
            'price_amount', p.amount
          )
        ) as "products!: Json<Vec<Product>>"
      from checkout_links cl
      join checkout_link_products clp on clp.checkout_link_id = cl.id
      join prices p on p.product_id = clp.product_id
      where cl.client_secret = $1
      group by cl.store_id, cl.success_url
    "#,
    &client_secret,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The checkout link '{client_secret}' could not be found."
    )
  })?;

  let products = checkout_link.products.0;
  let product = products.first().ok_or_else(|| failure!())?;

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let client_secret = create_client_secret("starfish_cs_");

  let checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      insert into checkout_sessions (
        store_id, 
        product_id, 
        client_secret, 
        amount
      )
      values (
        $1, 
        $2, 
        $3, 
        $4
      )
      returning
        id,
        store_id,
        product_id,
        customer_id as "customer_id: CustomerId",
        customer_email,
        client_secret,
        status as "status: CheckoutSessionStatus",
        rtrim($5, '/') || '/checkout/' || client_secret as "url!",
        success_url,
        amount,
        discount_amount,
        tax_amount,
        (amount - discount_amount) as "net_amount!",
        (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
        created_at,
        modified_at
    "#,
    &checkout_link.store_id,
    &product.id,
    &client_secret,
    product.price_amount,
    &state.config.website_base_url,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let product_identifiers = products
    .iter()
    .map(|product| product.id)
    .collect::<Vec<Uuid>>();

  sqlx::query!(
    r#"
      insert into checkout_session_products (checkout_session_id, product_id)
      select $1, products
      from unnest($2::uuid[]) as products
    "#,
    &checkout_session.id.0,
    &product_identifiers,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(checkout_session)
}
