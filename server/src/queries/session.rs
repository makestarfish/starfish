use crate::{
  context::RequestContext,
  entities::Session,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  id: Uuid,
) -> Result<Session, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let session = sqlx::query_as!(
    Session,
    r#"
      select 
        id, 
        user_id, 
        last_seen_at, 
        (id is not distinct from $3) as "is_current_session!"
      from sessions
      where id = $1 and user_id = $2 and revoked = false
    "#,
    id,
    &user_id,
    context.session_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The session '{id}' could not be found"
    )
  })?;

  Ok(session)
}
