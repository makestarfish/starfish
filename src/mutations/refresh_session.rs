use crate::{
  auth::{RefreshTokenClaims, SessionClaims},
  entities::Tokens,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  refresh_token: String,
) -> Result<Tokens, Failure> {
  let claims =
    RefreshTokenClaims::from_token(&refresh_token, &state.config.jwt_secret)
      .map_err(|_| {
        failure!(FailureReason::UNAUTHORIZED, "The refresh token is invalid")
      })?;

  let refresh_token = sqlx::query!(
    r#"
      select 
        r.revoked as refresh_token_revoked, 
        s.id as session_id, 
        s.user_id as session_user_id,
        s.revoked as session_revoked
      from refresh_tokens r
      join sessions s on r.session_id = s.id
      where r.id = $1
    "#,
    &claims.refresh_token_id
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  if refresh_token.refresh_token_revoked || refresh_token.session_revoked {
    bail!(FailureReason::FORBIDDEN, "The refresh token was revoked");
  }

  let mut tx = state.db.begin().await.map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update refresh_tokens
      set revoked = true
      where id = $1
    "#,
    &claims.refresh_token_id
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  let new_refresh_token = sqlx::query!(
    r#"
      insert into refresh_tokens (session_id)
      values ($1)
      returning id
    "#,
    &refresh_token.session_id
  )
  .fetch_one(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  sqlx::query!(
    r#"
      update sessions
      set last_seen_at = now()
      where id = $1
    "#,
    &refresh_token.session_id
  )
  .execute(&mut *tx)
  .await
  .map_err(|_| failure!())?;

  tx.commit().await.map_err(|_| failure!())?;

  let session_claims =
    SessionClaims::new(refresh_token.session_user_id, refresh_token.session_id);

  let refresh_token_claims = RefreshTokenClaims::new(new_refresh_token.id);

  Ok(Tokens {
    access_token: session_claims.encode(&state.config.jwt_secret),
    refresh_token: refresh_token_claims.encode(&state.config.jwt_secret),
    access_token_expires_at: session_claims.expires_at,
    refresh_token_expires_at: refresh_token_claims.expires_at,
  })
}
