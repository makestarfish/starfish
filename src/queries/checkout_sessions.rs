use crate::{
  context::RequestContext,
  entities::{
    CheckoutSession, CheckoutSessionConnection, CheckoutSessionEdge,
    CheckoutSessionStatus, CustomerId,
  },
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
) -> Result<CheckoutSessionConnection, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store = sqlx::query!(
    r#"
      select exists (
        select 1
        from store_members sm
        where sm.store_id = s.id and sm.user_id = $2
      ) as store_member_exists
      from stores s
      where s.id = $1
    "#,
    &store_id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The store '{}' could not be found",
      &store_id
    )
  })?;

  if !store.store_member_exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  let checkout_sessions = sqlx::query_as!(
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
        rtrim($4, '/') || '/checkout/' || client_secret as "url!",
        amount,
        discount_amount,
        tax_amount,
        (amount - discount_amount) as "net_amount!",
        (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
        created_at,
        modified_at
      from checkout_sessions
      where 
        store_id = $1 and
        (id < $2 or $2 is null) and 
        (id > $3 or $3 is null)
      order by id desc
      limit 20
    "#,
    &store_id,
    after,
    before,
    &state.config.website_base_url,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(CheckoutSessionConnection {
    edges: checkout_sessions
      .iter()
      .map(|checkout_session| CheckoutSessionEdge {
        cursor: checkout_session.id.to_owned(),
        node: checkout_session.to_owned(),
      })
      .collect(),
    nodes: checkout_sessions,
  })
}
