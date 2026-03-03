use crate::{
  context::RequestContext,
  entities::{StoreMember, StoreMemberConnection, StoreMemberEdge},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
) -> Result<StoreMemberConnection, Failure> {
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
    );
  }

  let store_members = sqlx::query_as!(
    StoreMember,
    r#"
      select id, store_id, user_id, created_at, modified_at
      from store_members
      where 
        store_id = $1 and 
        (id < $2 or $2 is null) and 
        (id > $3 or $3 is null)
      order by id desc
      limit 20
    "#,
    &store_id,
    after,
    before,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(StoreMemberConnection {
    edges: store_members
      .iter()
      .map(|member| StoreMemberEdge {
        cursor: member.id.to_owned(),
        node: member.clone(),
      })
      .collect(),
    nodes: store_members,
  })
}
