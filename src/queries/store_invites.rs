use crate::{
  context::RequestContext,
  entities::{StoreInvite, StoreInviteConnection, StoreInviteEdge},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
  revoked: Option<bool>,
) -> Result<StoreInviteConnection, Failure> {
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
    &user_id,
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

  let store_invites = sqlx::query_as!(
    StoreInvite,
    r#"
      select id, store_id, email, revoked_at, accepted_at, created_at, modified_at
      from store_invites
      where 
        store_id = $1 and
        (revoked_at is not null) = $2 and
        (id < $3 or $3 is null) and 
        (id > $4 or $4 is null)
      order by id desc
      limit 20
    "#,
    &store_id,
    revoked.unwrap_or(false),
    after,
    before,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(StoreInviteConnection {
    edges: store_invites
      .iter()
      .map(|store_invite| StoreInviteEdge {
        cursor: store_invite.id.to_owned(),
        node: store_invite.clone(),
      })
      .collect(),
    nodes: store_invites,
  })
}
