use crate::{
  entities::{CheckoutSession, CheckoutSessionStatus, CustomerId},
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  client_secret: String,
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
        amount,
        discount_amount,
        tax_amount,
        (amount - discount_amount) as "net_amount!",
        (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
        created_at,
        modified_at
      from checkout_sessions
      where client_secret = $1
    "#,
    &client_secret,
    &state.config.website_base_url,
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

  Ok(checkout_session)
}
