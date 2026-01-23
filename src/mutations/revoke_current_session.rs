use crate::{
  context::RequestContext,
  entities::RevokedSession,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  request: &RequestContext,
) -> Result<RevokedSession, Failure> {
  let session_id = request
    .session_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let session = sqlx::query_as!(
    RevokedSession,
    r#"
      update sessions
      set revoked = true
      where id = $1
      returning id
    "#,
    &session_id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(session)
}
