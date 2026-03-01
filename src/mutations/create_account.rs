use crate::{
  context::RequestContext,
  entities::{Account, AccountStatus, StoreStatus},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use starfish_stripe::types::{
  CreateAccountCapabilities, CreateAccountCapability, CreateAccountParams,
  CreateAccountType,
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

  let store = sqlx::query!(
    r#"
      select 
        s.status as "status: StoreStatus",
        exists (
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
      "The store '{store_id}' could not be found"
    )
  })?;

  if !store.store_member_exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  if store.status != StoreStatus::Created {
    bail!(FailureReason::CONFLICT, "This store already has an account")
  }

  let account_transfers = CreateAccountCapability::new().with_requested(true);

  let account_capabilities =
    CreateAccountCapabilities::new().with_transfers(account_transfers);

  let create_account_params = CreateAccountParams::new()
    .with_type(CreateAccountType::Express)
    .with_capabilities(account_capabilities);

  let stripe_account = state
    .stripe
    .accounts
    .create(create_account_params)
    .await
    .map_err(|_| failure!())?;

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update stores
      set status = 'onboarding_started'
      where id = $1
    "#,
    &store_id
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let account = sqlx::query_as!(
    Account,
    r#"
      insert into accounts (store_id, stripe_id)
      values ($1, $2)
      returning id, stripe_id, status as "status: AccountStatus"
    "#,
    &store_id,
    &stripe_account.id
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  Ok(account)
}
