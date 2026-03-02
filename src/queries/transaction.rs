use crate::{
  context::RequestContext,
  entities::{Transaction, TransactionId},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<Transaction, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(crate::failure::FailureReason::UNAUTHORIZED))?;

  let transaction = sqlx::query_as!(
    Transaction,
    r#"
      select 
        t.id, 
        t.incurred_by as "incurred_by: TransactionId",
        t.account_id,
        t.order_id, 
        t.amount, 
        t.incurred_amount, 
        (t.amount + t.incurred_amount) as "net_amount!",
        t.created_at,
        t.modified_at
      from transactions t
      join accounts a on t.account_id = a.id
      where 
        t.id = $1 and
        exists (
          select 1
          from store_members sm
          where sm.store_id = a.store_id and sm.user_id = $2
        )
    "#,
    &id,
    &user_id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The transaction '{id}' could not be found"
    )
  })?;

  Ok(transaction)
}
