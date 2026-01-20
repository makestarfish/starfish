use crate::auth::SessionClaims;
use axum::http::HeaderMap;
use uuid::Uuid;

#[derive(Default)]
pub struct RequestContext {
  pub user_id: Option<Uuid>,
  pub session_id: Option<Uuid>,
}

impl RequestContext {
  pub fn from_headers(headers: HeaderMap, token_signing_secret: &str) -> Self {
    headers
      .get("authorization")
      .and_then(|value| value.to_str().ok())
      .and_then(|authorization| authorization.strip_prefix("Bearer "))
      .and_then(|access_token| {
        SessionClaims::from_token(access_token, token_signing_secret)
          .map(|claims| Self {
            user_id: Some(claims.user_id),
            session_id: Some(claims.session_id),
          })
          .ok()
      })
      .unwrap_or_default()
  }
}
