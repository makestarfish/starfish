use crate::{
  context::RequestContext,
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::create_client_secret,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  products: Vec<Uuid>,
  customer_id: Option<Uuid>,
  customer_email: Option<String>,
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
    bail!(
      FailureReason::NOT_FOUND,
      "The product '{}' could not be found",
      products
        .iter()
        .find(|&id| product_data.iter().all(|data| data.id != *id))
        .unwrap()
    )
  }

  let product = product_data
    .iter()
    .find(|&data| data.id == *products.first().unwrap())
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

  if let Some(customer_id) = customer_id {
    let customer = sqlx::query!(
      r#"
        select email
        from customers
        where id = $1 and store_id = $2
      "#,
      &customer_id,
      &product.store_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| failure!())?
    .ok_or_else(|| {
      failure!(
        FailureReason::NOT_FOUND,
        "The customer '{customer_id}' could not be found"
      )
    })?;

    if let Some(customer_email) = customer_email.as_ref()
      && *customer_email != customer.email
    {
      bail!(
        FailureReason::CONFLICT,
        "The 'customer_email' argument does not match the email of the customer '{customer_id}'"
      )
    }
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let client_secret = create_client_secret("starfish_cs_");

  let checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      insert into checkout_sessions (
        store_id,
        product_id, 
        client_secret,
        customer_id, 
        amount, 
        customer_email
      )
      values ($1, $2, $3, $4, $5, $6)
      returning
        id,
        store_id,
        product_id,
        customer_id as "customer_id: CustomerId",
        customer_email,
        client_secret,
        status as "status: CheckoutSessionStatus",
        rtrim($7, '/') || '/checkout/' || client_secret as "url!",
        success_url,
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
    &client_secret,
    customer_id,
    product.price_amount,
    customer_email,
    &state.config.website_base_url,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      insert into checkout_session_products (checkout_session_id, product_id)
      select $1, products
      from unnest($2::uuid[]) as products
    "#,
    &checkout_session.id.0,
    &products,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(checkout_session)
}
