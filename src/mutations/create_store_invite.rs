use crate::{
  context::RequestContext,
  entities::StoreInvite,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  email: String,
) -> Result<StoreInvite, Failure> {
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
    &store_id,
    &user_id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !store_member.exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  let store_invite = sqlx::query_as!(
    StoreInvite,
    r#"
      insert into store_invites (store_id, inviter_id, email)
      values ($1, $2, $3)
      returning
        id,
        store_id,
        email,
        accepted_at,
        revoked_at,
        created_at, 
        modified_at
    "#,
    &store_id,
    &user_id,
    &email
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(store_invite)
}
