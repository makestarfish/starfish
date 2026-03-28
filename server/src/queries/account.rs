use crate::{
  context::RequestContext,
  entities::{Account, AccountStatus},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
) -> Result<Option<Account>, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let account = sqlx::query_as!(
    Account,
    r#"
      select a.id, a.stripe_id, a.status as "status: AccountStatus"
      from accounts a
      where 
        a.store_id = $1 and
        exists (
          select 1
          from store_members sm
          where a.store_id = sm.store_id and sm.user_id = $2
        )
    "#,
    &store_id,
    &user_id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(account)
}
