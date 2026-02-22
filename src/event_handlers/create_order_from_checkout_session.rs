use crate::{
  entities::{BillingReason, CheckoutSessionStatus, OrderStatus},
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::calculate_platform_fee,
};
use starfish_stripe::types::Event;

pub async fn handle(state: &SharedState, event: Event) -> Result<(), Failure> {
  let payment_intent_id = event.data.object.get("id").unwrap();

  let checkout_session = sqlx::query!(
    r#"
      select 
        id, 
        store_id, 
        customer_id as "customer_id!", 
        status as "status: CheckoutSessionStatus",
        amount,
        tax_amount,
        discount_amount
      from checkout_sessions
      where stripe_id = $1
    "#,
    payment_intent_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The checkout session could not be found"
    )
  })?;

  if matches!(checkout_session.status, CheckoutSessionStatus::Succeeded) {
    return Ok(());
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update checkout_sessions
      set 
        status = 'succeeded',
        modified_at = now()
      where stripe_id = $1
    "#,
    &payment_intent_id
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      insert into orders (
        store_id, 
        customer_id, 
        checkout_session_id, 
        status, 
        subtotal_amount, 
        discount_amount, 
        tax_amount, 
        platform_fee_amount, 
        billing_reason
      )
      values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    "#,
    &checkout_session.store_id,
    &checkout_session.customer_id,
    &checkout_session.id,
    OrderStatus::Paid as OrderStatus,
    checkout_session.amount,
    checkout_session.discount_amount,
    checkout_session.tax_amount,
    calculate_platform_fee(checkout_session.amount),
    BillingReason::Purchase as BillingReason,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(())
}
