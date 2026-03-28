use uuid::Uuid;

use crate::{
  context::RequestContext,
  entities::{
    BillingReason, CheckoutSessionId, Order, OrderConnection, OrderEdge,
    OrderStatus,
  },
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
) -> Result<OrderConnection, Failure> {
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
    &user_id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The store '{store_id}' could not be found"
    )
  })?;

  if !store.store_member_exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  let orders = sqlx::query_as!(
    Order,
    r#"
      select 
        id,
        store_id,
        customer_id,
        checkout_session_id as "checkout_session_id: CheckoutSessionId",
        status as "status: OrderStatus",
        subtotal_amount,
        discount_amount,
        (subtotal_amount - discount_amount) as "net_amount!",
        tax_amount,
        (subtotal_amount - discount_amount + tax_amount) as "total_amount!",
        platform_fee_amount,
        billing_reason as "billing_reason: BillingReason",
        created_at,
        modified_at
      from orders o
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
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(OrderConnection {
    edges: orders
      .iter()
      .map(|order| OrderEdge {
        cursor: order.id.to_owned(),
        node: order.to_owned(),
      })
      .collect(),
    nodes: orders,
  })
}
