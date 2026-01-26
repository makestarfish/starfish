use uuid::Uuid;

use crate::{
  context::RequestContext,
  entities::{Customer, CustomerConnection, CustomerEdge, Store},
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store: &Store,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
) -> Result<CustomerConnection, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store_member = sqlx::query!(
    r#"
      select exists (
        select 1
        from store_members
        where store_id = $1 and user_id = $2
      )
    "#,
    &store.id.0,
    &user_id,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !store_member.exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    );
  }

  let customers = sqlx::query_as!(
    Customer,
    r#"
      select id, store_id, email, name, created_at, modified_at
      from customers
      where 
        store_id = $1 and
        deleted_at is null and
        (id < $2 or $2 is null) and 
        (id > $3 or $3 is null)
      order by id desc
      limit 20
    "#,
    &store.id.0,
    after,
    before,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(CustomerConnection {
    edges: customers
      .iter()
      .map(|customer| CustomerEdge {
        cursor: customer.id.to_owned(),
        node: customer.clone(),
      })
      .collect(),
    nodes: customers,
  })
}
