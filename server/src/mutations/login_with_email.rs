use crate::{
  auth::{RefreshTokenClaims, SessionClaims},
  entities::Tokens,
  failure::{Failure, FailureReason},
  state::SharedState,
  utils::create_hash,
};
use uuid::Uuid;

pub async fn resolve(
  state: SharedState,
  one_time_token_id: Uuid,
  email: String,
  code: String,
) -> Result<Tokens, Failure> {
  let code_hash = create_hash(&code);

  let one_time_token = sqlx::query!(
    r#"
      select exists (
        select 1
        from one_time_tokens
        where 
          id = $1 and 
          email = $2 and
          code_hash = $3 and 
          expires_at > now() 
          and was_used = false
      )
    "#,
    &one_time_token_id,
    &email,
    &code_hash,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if one_time_token.exists.is_none_or(|exists| !exists) {
    bail!(
      FailureReason::NOT_FOUND,
      "There's no matching one-time token"
    )
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update one_time_tokens
      set was_used = true
      where id = $1
    "#,
    &one_time_token_id,
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let user_id = {
    let user = sqlx::query!(
      r#"
        select id
        from users
        where email = $1
      "#,
      &email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| failure!())?;

    if let Some(user) = user {
      user.id
    } else {
      let user = sqlx::query!(
        r#"
          insert into users (email)
          values ($1)
          returning id
        "#,
        &email,
      )
      .fetch_one(&mut *tx)
      .await
      .map_err(|_| failure!())?;

      user.id
    }
  };

  let session = sqlx::query!(
    r#"
      insert into sessions (user_id)
      values ($1)
      returning id
    "#,
    &user_id
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let refresh_token = sqlx::query!(
    r#"
      insert into refresh_tokens (session_id)
      values ($1)
      returning id
    "#,
    &session.id,
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  let session_claims = SessionClaims::new(user_id, session.id);
  let refresh_token_claims = RefreshTokenClaims::new(refresh_token.id);

  Ok(Tokens {
    access_token: session_claims.encode(&state.config.jwt_secret),
    refresh_token: refresh_token_claims.encode(&state.config.jwt_secret),
    access_token_expires_at: session_claims.expires_at,
    refresh_token_expires_at: refresh_token_claims.expires_at,
  })
}
