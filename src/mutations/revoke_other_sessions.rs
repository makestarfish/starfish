use crate::{
  context::RequestContext,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
) -> Result<bool, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  sqlx::query!(
    r#"
      update sessions
      set revoked = true
      where user_id = $1 and id != $2
    "#,
    &user_id,
    context.session_id
  )
  .execute(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(true)
}
