use crate::{
  context::RequestContext,
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<Option<CheckoutSession>, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let checkout_session = sqlx::query_as!(
    CheckoutSession,
    r#"
      select 
        cs.id, 
        cs.store_id, 
        cs.product_id,
        cs.customer_id as "customer_id: CustomerId",
        cs.customer_email,
        cs.client_secret,
        cs.status as "status: CheckoutSessionStatus",
        rtrim($3, '/') || '/checkout' || cs.client_secret as "url!",
        cs.amount,
        cs.discount_amount,
        cs.tax_amount,
        (cs.amount - cs.discount_amount) as "net_amount!",
        (cs.amount - cs.discount_amount + coalesce(cs.tax_amount, 0)) as "total_amount!",
        cs.created_at,
        cs.modified_at
      from checkout_sessions cs
      where 
        cs.id = $1 and 
        exists (
          select 1
          from store_members sm
          where sm.store_id = cs.store_id and sm.user_id = $2
        )
    "#,
    &id,
    &user_id,
    &state.config.website_base_url,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(checkout_session)
}
