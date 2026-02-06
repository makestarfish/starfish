use crate::{
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use starfish_stripe::types::CreateCustomerParams;

pub async fn resolve(
  state: &SharedState,
  client_secret: String,
  _confirmation_token_id: String,
  customer_email: Option<String>,
) -> Result<CheckoutSession, Failure> {
  let checkout_session = sqlx::query!(
    r#"
      select
        id,
        stripe_id,
        store_id,
        customer_id,
        customer_email
      from checkout_sessions
      where client_secret = $1
    "#,
    &client_secret,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The checkout session '{client_secret}' could not be found"
    )
  })?;

  if checkout_session.customer_email.is_some()
    && customer_email.is_some()
    && customer_email != checkout_session.customer_email
  {
    bail!(
      FailureReason::UNPROCESSABLE_ENTITY,
      "The customer email is immutable once set"
    )
  }

  let customer_email = checkout_session.customer_email
    .or(customer_email)
    .ok_or_else(|| failure!(FailureReason::UNPROCESSABLE_ENTITY, "The argument 'customer_email' is required since the checkout session is not associated with a customer"))?;

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let customer_id = match checkout_session.customer_id {
    Some(customer_id) => customer_id,
    _ => 'customer: {
      let customer_with_same_email = sqlx::query!(
        r#"
          select id
          from customers
          where store_id = $1 and email = $2
        "#,
        &checkout_session.store_id,
        &customer_email,
      )
      .fetch_optional(&mut *tx)
      .await
      .map_err(|_| failure!())?;

      if let Some(existing_customer) = customer_with_same_email {
        break 'customer existing_customer.id;
      }

      let create_customer_params =
        CreateCustomerParams::new().with_email(&customer_email);

      let stripe_customer = state
        .stripe
        .customers
        .create(create_customer_params)
        .await
        .map_err(|_| failure!())?;

      let created_customer = sqlx::query!(
        r#"
          insert into customers (stripe_id, store_id, email)
          values ($1, $2, $3)
          returning id
        "#,
        &stripe_customer.id,
        &checkout_session.store_id,
        &customer_email,
      )
      .fetch_one(&mut *tx)
      .await
      .map_err(|_| failure!())?;

      created_customer.id
    }
  };

  let confirmed_checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      update checkout_sessions
      set 
        customer_id = $2,
        customer_email = $3,
        status = 'confirmed'
      where id = $1
      returning
        id, 
        store_id, 
        product_id,
        customer_id as "customer_id: CustomerId",
        customer_email,
        client_secret,
        status as "status: CheckoutSessionStatus",
        amount,
        discount_amount,
        tax_amount,
        (amount - discount_amount) as "net_amount!",
        (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
        created_at,
        modified_at
    "#,
    &checkout_session.id,
    &customer_id,
    &customer_email
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(confirmed_checkout_session)
}
