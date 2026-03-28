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
  id: Uuid,
) -> Result<StoreInvite, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store_invite = sqlx::query_as!(
    StoreInvite,
    r#"
      select
        si.id, 
        si.store_id,
        si.email, 
        si.accepted_at, 
        si.revoked_at, 
        si.created_at, 
        si.modified_at
      from store_invites si
      where si.id = $1 and exists (
        select 1
        from store_members sm
        where sm.store_id = si.store_id and sm.user_id = $2
      )
    "#,
    &id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The store invite '{id}' could not be found"
    )
  })?;

  if store_invite.revoked_at.is_some() {
    return Ok(store_invite);
  }

  let revoked_store_invite = sqlx::query_as!(
    StoreInvite,
    r#"
      update store_invites
      set revoked_at = now(), modified_at = now()
      where id = $1
      returning 
        id, 
        store_id, 
        email, 
        accepted_at, 
        revoked_at, 
        created_at, 
        modified_at
    "#,
    &id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(revoked_store_invite)
}
