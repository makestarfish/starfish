use crate::{
  context::RequestContext,
  entities::User,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
) -> Result<User, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let user = sqlx::query_as!(
    User,
    r#"
      select id, email, name, avatar_url, created_at, modified_at
      from users
      where id = $1
    "#,
    &user_id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(user)
}
