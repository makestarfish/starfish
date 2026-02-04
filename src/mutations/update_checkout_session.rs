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
  product_id: Option<Uuid>,
) -> Result<CheckoutSession, Failure> {
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
        cs.status as "status: CheckoutSessionStatus",
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
  ).fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| failure!(FailureReason::NOT_FOUND, "The checkout session '{id}' could not be found"))?;

  if let Some(product_id) = product_id
    && checkout_session.product_id.0 != product_id
  {
    let price = sqlx::query!(
      r#"
        select p.amount
        from prices p
        where 
          p.product_id = $1 and 
          exists (
            select 1
            from checkout_session_products c
            where c.product_id = p.product_id and c.checkout_session_id = $2
          )
      "#,
      &product_id,
      &id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| failure!())?
    .ok_or_else(|| {
      failure!(
        FailureReason::CONFLICT,
        "The product '{product_id}' does not belong to this checkout session"
      )
    })?;

    let updated_checkout_session = sqlx::query_as!(
      CheckoutSession,
      r#"
        update checkout_sessions
        set 
          product_id = $2, 
          amount = $3,
          modified_at = now()
        where id = $1
        returning
          id,
          store_id,
          product_id,
          customer_id as "customer_id: CustomerId",
          customer_email,
          status as "status: CheckoutSessionStatus",
          amount,
          discount_amount,
          tax_amount,
          (amount - discount_amount) as "net_amount!",
          (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
          created_at,
          modified_at
      "#,
      &id,
      &product_id,
      price.amount
    ).fetch_one(&state.db).await.map_err(|_| failure!())?;

    return Ok(updated_checkout_session);
  }

  Ok(checkout_session)
}
