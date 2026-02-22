use crate::{
  context::RequestContext,
  entities::Account,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
) -> Result<Account, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let account = sqlx::query_as!(
    Account,
    r#"
      select a.id, a.stripe_id
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
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The account of the store '{store_id}' could not be found"
    )
  })?;

  Ok(account)
}
