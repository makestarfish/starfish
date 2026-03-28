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
) -> Result<Option<StoreInvite>, Failure> {
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
      where 
        si.id = $1 and
        exists (
          select 1
          from store_members sm
          where sm.store_id = si.store_id and sm.user_id = $2
        )
    "#,
    &id,
    &user_id
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(store_invite)
}
