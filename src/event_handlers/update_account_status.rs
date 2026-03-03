use crate::{
  failure::{Failure, FailureReason},
  state::SharedState,
};
use starfish_stripe::types::Account;

pub async fn handle(
  state: &SharedState,
  stripe_account: Account,
) -> Result<(), Failure> {
  let account = sqlx::query!(
    r#"
      select id, store_id
      from accounts
      where stripe_id = $1
    "#,
    &stripe_account.id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The account '{}' could not be found",
      stripe_account.id
    )
  })?;

  let is_account_active = &stripe_account
    .requirements
    .is_some_and(|r| r.disabled_reason.is_none());

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update accounts
      set status = case 
        when $2 then 'active'::account_status
        else 'onboarding_started'::account_status
      end
      where id = $1
    "#,
    &account.id,
    &is_account_active,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update stores
      set status = case
        when $2 then 'active'::store_status
        else 'onboarding_started'::store_status
      end
      where id = $1
    "#,
    &account.store_id,
    &is_account_active
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(())
}
