use crate::{
  context::RequestContext,
  entities::{BillingReason, CheckoutSessionId, Order, OrderStatus},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<Order, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let order = sqlx::query_as!(
    Order,
    r#"
      select 
        o.id,
        o.store_id,
        o.customer_id,
        o.checkout_session_id as "checkout_session_id: CheckoutSessionId",
        o.status as "status: OrderStatus",
        o.subtotal_amount,
        o.discount_amount,
        (o.subtotal_amount - o.discount_amount) as "net_amount!",
        o.tax_amount,
        (o.subtotal_amount - o.discount_amount + o.tax_amount) as "total_amount!",
        o.platform_fee_amount,
        o.billing_reason as "billing_reason: BillingReason",
        o.created_at,
        o.modified_at
      from orders o
      where 
        o.id = $1 and 
        exists (
          select 1
          from store_members sm
          where sm.store_id = o.store_id and sm.user_id = $2
        )
    "#,
    &id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| failure!(FailureReason::NOT_FOUND, "The order '{id}' could not be found"))?;

  Ok(order)
}
