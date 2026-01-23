use crate::{
  context::RequestContext,
  entities::RevokedSession,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  request: &RequestContext,
  id: Uuid,
) -> Result<RevokedSession, Failure> {
  let user_id = request
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let session = sqlx::query!(
    r#"
      select exists (
        select 1
        from sessions
        where user_id = $1 and id = $2
      )
    "#,
    &user_id,
    &id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if !session.exists.unwrap_or_default() {
    bail!(
      FailureReason::NOT_FOUND,
      "The session '{id}' could not be found"
    );
  }

  let session = sqlx::query_as!(
    RevokedSession,
    r#"
      update sessions
      set revoked = true
      where id = $1
      returning id
    "#,
    &id,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(session)
}
