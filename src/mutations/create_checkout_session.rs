use crate::{
  context::RequestContext,
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use sqlx::QueryBuilder;
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  products: Vec<Uuid>,
) -> Result<CheckoutSession, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let product_data = sqlx::query!(
    r#"
      select products.id, products.store_id, prices.amount as price_amount
      from products
      join prices on prices.product_id = products.id
      where 
        products.id = any($1) and
        exists (
          select 1
          from store_members
          where 
            store_members.store_id = products.store_id and 
            store_members.user_id = $2
        )
    "#,
    &products,
    &user_id,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  if product_data.len() != products.len() {
    let first_missing_product = products
      .iter()
      .find(|&id| !product_data.iter().any(|data| data.id == *id))
      .unwrap();

    bail!(
      FailureReason::NOT_FOUND,
      "The product '{}' could not be found",
      first_missing_product
    )
  }

  let product = product_data
    .iter()
    .find(|&data| data.id == *products.first().unwrap())
    .unwrap();

  if let Some(product_from_other_store) = product_data
    .iter()
    .find(|data| data.store_id != product.store_id)
  {
    bail!(
      FailureReason::CONFLICT,
      "The products '{}' and '{}' are from different stores",
      product.id,
      product_from_other_store.id
    )
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      insert into checkout_sessions (store_id, product_id, amount)
      values ($1, $2, $3)
      returning
        id,
        store_id,
        product_id,
        customer_id as "customer_id: CustomerId",
        status as "status: CheckoutSessionStatus",
        amount,
        discount_amount,
        tax_amount,
        (amount - discount_amount) as "net_amount!",
        (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
        created_at,
        modified_at
    "#,
    &product.store_id,
    &product.id,
    product.price_amount,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let mut query_builder = QueryBuilder::new(
    "insert into checkout_session_products (checkout_session_id, product_id)",
  );

  query_builder.push_values(&products, |mut b, product_id| {
    b.push_bind(&checkout_session.id).push_bind(product_id);
  });

  query_builder
    .build()
    .execute(&mut *tx)
    .await
    .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(checkout_session)
}
