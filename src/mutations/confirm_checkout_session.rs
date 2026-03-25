use crate::{
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::create_avatar_url,
};
use starfish_stripe::types::ConfirmPaymentIntentParams;

pub async fn resolve(
  state: &SharedState,
  client_secret: String,
  confirmation_token_id: String,
  customer_email: Option<String>,
) -> Result<CheckoutSession, Failure> {
  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let checkout_session = sqlx::query!(
    r#"
      select
        id,
        stripe_id,
        store_id,
        customer_id,
        customer_email,
        success_url
      from checkout_sessions
      where client_secret = $1
      for update
    "#,
    &client_secret,
  )
  .fetch_optional(&mut *tx)
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

  let customer_id = match checkout_session.customer_id {
    Some(customer_id) => customer_id,
    _ => {
      let customer = sqlx::query!(
        r#"
          insert into customers (store_id, email, avatar_url)
          values ($1, $2, $3)
          on conflict (store_id, email) do update set email = excluded.email
          returning id
        "#,
        &checkout_session.store_id,
        &customer_email,
        create_avatar_url(&customer_email),
      )
      .fetch_one(&mut *tx)
      .await
      .map_err(|_| failure!())?;

      customer.id
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
        rtrim($4, '/') || '/checkout/' || client_secret as "url!",
        success_url,
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
    &customer_email,
    &state.config.website_base_url,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  let confirm_params = ConfirmPaymentIntentParams::new()
    .with_confirmation_token(&confirmation_token_id)
    .with_return_url(&state.config.website_base_url);

  state
    .stripe
    .payment_intents
    .confirm(&checkout_session.stripe_id, confirm_params)
    .await
    .map_err(|_| failure!())?;

  Ok(confirmed_checkout_session)
}
