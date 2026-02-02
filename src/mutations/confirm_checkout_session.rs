use crate::{
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use starfish_stripe::types::CreatePaymentIntentParams;
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  id: Uuid,
  confirmation_token_id: String,
) -> Result<CheckoutSession, Failure> {
  let checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#" 
      select
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
      from checkout_sessions
      where id = $1
    "#,
    &id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The checkout session '{id}' could not be found"
    )
  })?;

  let create_payment_intent_params =
    CreatePaymentIntentParams::new(checkout_session.total_amount, "usd")
      .with_confirm(true)
      .with_confirmation_token(&confirmation_token_id);

  let _payment_intent = state
    .stripe
    .payment_intents
    .create(create_payment_intent_params)
    .await
    .map_err(|_| failure!())?;

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let confirmed_checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      update checkout_sessions
      set status = 'confirmed'
      where id = $1
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
    &checkout_session.id.0,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(confirmed_checkout_session)
}
