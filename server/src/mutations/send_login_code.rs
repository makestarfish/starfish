use crate::{
  entities::OneTimeToken,
  failure::Failure,
  state::SharedState,
  utils::{create_hash, create_random_code},
};
use chrono::{Duration, Utc};
use resend_rs::types::CreateEmailBaseOptions;

pub async fn resolve(
  state: SharedState,
  email: String,
) -> Result<OneTimeToken, Failure> {
  let code = create_random_code();
  let code_hash = create_hash(&code);
  let expires_at = Utc::now() + Duration::minutes(5);

  let one_time_token = sqlx::query_as!(
    OneTimeToken,
    r#"
      insert into one_time_tokens (email, code_hash, expires_at)
      values ($1, $2, $3)
      returning id, email, expires_at
    "#,
    &email,
    &code_hash,
    expires_at,
  )
  .fetch_one(&state.db)
  .await
  .map_err(|_| failure!())?;

  let create_email_options = CreateEmailBaseOptions::new(
    "Starfish <starfish@dmelo.sh>",
    [&email],
    "Login to Starfish",
  )
  .with_html(&format!("Your one-time token is {}.", &code));

  state
    .resend
    .emails
    .send(create_email_options)
    .await
    .map_err(|_| failure!())?;

  Ok(one_time_token)
}
