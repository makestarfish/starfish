use crate::{
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::create_avatar_url,
};
use sqlx::{Postgres, Transaction};
use starfish_stripe::types::CreatePaymentIntentParams;

pub async fn resolve(
  state: &SharedState,
  client_secret: String,
  confirmation_token_id: String,
  customer_email: Option<String>,
) -> Result<CheckoutSession, Failure> {
  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let checkout_session =
    lock_checkout_session_update(state, &mut tx, &client_secret).await?;

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

  let customer = sqlx::query!(
    r#"
      insert into customers (store_id, email, avatar_url)
      values ($1, $2, $3)
      on conflict (store_id, email) 
        do update set email = excluded.email
      returning id
    "#,
    &checkout_session.store_id.0,
    &customer_email,
    create_avatar_url(&customer_email),
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

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
    &checkout_session.id.0,
    &customer.id,
    &customer_email,
    &state.config.website_base_url,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let create_payment_intent_params =
    CreatePaymentIntentParams::new(checkout_session.total_amount, "usd")
      .with_confirmation_token(&confirmation_token_id)
      .with_confirm(true)
      .with_metadata("store_id", &checkout_session.store_id.0.to_string())
      .with_metadata("checkout_session_id", &checkout_session.id.0.to_string());

  let create_payment_intent_result = state
    .stripe
    .payment_intents
    .create(create_payment_intent_params)
    .await;

  if let Err(error) = create_payment_intent_result {
    if let starfish_stripe::Error::Stripe(stripe_error) = error
      && stripe_error.decline_code.is_some()
    {
      bail!(
        FailureReason::BAD_REQUEST,
        "{}",
        stripe_error
          .message
          .unwrap_or("The checkout session could not be processed".to_owned())
      )
    }

    bail!()
  }

  tx.commit().await.map_err(|_| failure!())?;

  Ok(confirmed_checkout_session)
}

/// Locks the checkout session using FOR UPDATE NO WAIT;
/// See: https://www.postgresql.org/docs/current/explicit-locking.html
async fn lock_checkout_session_update(
  state: &SharedState,
  tx: &mut Transaction<'_, Postgres>,
  client_secret: &str,
) -> Result<CheckoutSession, Failure> {
  let checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      select
        id,
        store_id,
        product_id,
        customer_id as "customer_id: CustomerId",
        customer_email,
        client_secret,
        status as "status: CheckoutSessionStatus",
        rtrim($2, '/') || '/checkout/' || client_secret as "url!",
        success_url,
        amount,
        discount_amount,
        tax_amount,
        (amount - discount_amount) as "net_amount!",
        (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
        created_at,
        modified_at
      from checkout_sessions
      where client_secret = $1
      for update of checkout_sessions nowait
    "#,
    client_secret,
    &state.config.website_base_url,
  )
  .fetch_optional(&mut **tx)
  .await
  .map_err(|error| error.into_database_error());

  match checkout_session {
    Err(error) => {
      // The 'lock_not_available' error.
      // See: https://www.postgresql.org/docs/current/errcodes-appendix.html
      if error.is_some_and(|e| e.code().is_some_and(|c| c == "55P03")) {
        bail!(
          FailureReason::CONFLICT,
          "The checkout session is being processed"
        )
      }

      bail!()
    }
    Ok(checkout_session) => {
      if let Some(checkout_session) = checkout_session {
        Ok(checkout_session)
      } else {
        bail!(
          FailureReason::NOT_FOUND,
          "The checkout session could not be found"
        )
      }
    }
  }
}
