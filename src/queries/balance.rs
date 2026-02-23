use crate::{
  context::RequestContext,
  entities::Balance,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  account_id: Uuid,
) -> Result<Balance, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(crate::failure::FailureReason::UNAUTHORIZED))?;

  let balance = sqlx::query_as!(
    Balance,
    r#"
      select b.amount 
      from balances b
      join accounts a on b.account_id = a.id
      where 
        b.account_id = $1 and 
        exists (
          select 1
          from store_members sm
          where sm.store_id = a.store_id and sm.user_id = $2
        )
    "#,
    &account_id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The account '{account_id}' could not be found"
    )
  })?;

  Ok(balance)
}
