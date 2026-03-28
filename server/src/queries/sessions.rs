use crate::{
  context::RequestContext,
  entities::Session,
  failure::{Failure, FailureReason},
  state::SharedState,
};

// TODO: implement pagination
pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
) -> Result<Vec<Session>, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let sessions = sqlx::query_as!(
    Session,
    r#"
      select 
        id, 
        user_id, 
        last_seen_at, 
        (id is not distinct from $2) as "is_current_session!"
      from sessions
      where user_id = $1 and revoked = false
    "#,
    &user_id,
    context.session_id
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(sessions)
}
