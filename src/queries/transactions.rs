use crate::{
  context::RequestContext,
  entities::{Transaction, TransactionConnection, TransactionEdge},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  account_id: Uuid,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
) -> Result<TransactionConnection, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let account = sqlx::query!(
    r#"
      select exists (
        select 1
        from accounts a
        where 
          a.id = $1
          and exists (
            select 1
            from store_members sm
            where sm.store_id = a.store_id and sm.user_id = $2
          )
      )
    "#,
    &account_id,
    &user_id,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !account.exists.unwrap_or_default() {
    bail!(
      FailureReason::NOT_FOUND,
      "The account '{account_id}' could not be found"
    )
  }

  let transactions = sqlx::query_as!(
    Transaction,
    r#"
      select 
        id, 
        account_id,
        amount, 
        fee_amount,
        (amount - fee_amount) as "net_amount!",
        created_at
      from transactions
      where 
        account_id = $1 and
        (id < $2 or $2 is null) and 
        (id > $3 or $3 is null)
      order by id desc
      limit 20
    "#,
    &account_id,
    after,
    before,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(TransactionConnection {
    edges: transactions
      .iter()
      .map(|transaction| TransactionEdge {
        cursor: transaction.id.to_owned(),
        node: transaction.to_owned(),
      })
      .collect(),
    nodes: transactions,
  })
}
