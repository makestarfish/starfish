use crate::{
  entities::{BillingReason, CheckoutSessionStatus, OrderStatus},
  failure::Failure,
  state::SharedState,
  utils::calculate_platform_fee,
};
use uuid::Uuid;

pub async fn handle(
  state: &SharedState,
  checkout_session_id: Uuid,
) -> Result<(), Failure> {
  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  let checkout_session = sqlx::query!(
    r#"
      select 
        cs.id, 
        cs.store_id, 
        cs.customer_id as "customer_id!", 
        cs.status as "status: CheckoutSessionStatus",
        cs.amount,
        cs.tax_amount,
        cs.discount_amount,
        (cs.amount - cs.discount_amount) as "net_amount!",
        a.id as store_account_id
      from checkout_sessions cs
      join accounts a on a.store_id = cs.store_id
      where cs.id = $1
      for update
    "#,
    &checkout_session_id,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  if matches!(checkout_session.status, CheckoutSessionStatus::Succeeded) {
    return Ok(());
  }

  sqlx::query!(
    r#"
      update checkout_sessions
      set 
        status = 'succeeded',
        modified_at = now()
      where id = $1
    "#,
    &checkout_session_id,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let order = sqlx::query!(
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
      returning id, platform_fee_amount
    "#,
    &checkout_session.store_id,
    &checkout_session.customer_id,
    &checkout_session.id,
    OrderStatus::Paid as OrderStatus,
    checkout_session.amount,
    checkout_session.discount_amount,
    checkout_session.tax_amount.unwrap_or(0),
    calculate_platform_fee(checkout_session.net_amount),
    BillingReason::Purchase as BillingReason,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let transaction = sqlx::query!(
    r#"
      insert into transactions (account_id, order_id, amount, incurred_amount)
      values ($1, $2, $3, $4)
      returning id
    "#,
    &checkout_session.store_account_id,
    &order.id,
    checkout_session.net_amount,
    -order.platform_fee_amount,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      insert into transactions (account_id, order_id, incurred_by, amount, incurred_amount)
      values ($1, $2, $3, $4, $5)
    "#,
    &checkout_session.store_account_id,
    &order.id,
    &transaction.id,
    -order.platform_fee_amount,
    0,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update balances
      set amount = amount + $2
      where account_id = $1
    "#,
    checkout_session.store_account_id,
    checkout_session.net_amount - order.platform_fee_amount,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(())
}
