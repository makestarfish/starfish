use chrono::DateTime;
use starfish_stripe::types::{AccountLinkType, CreateAccountLinkParams};
use uuid::Uuid;

use crate::{
  context::RequestContext,
  entities::OnboardingLink,
  failure::{Failure, FailureReason},
  state::SharedState,
};

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  account_id: Uuid,
) -> Result<OnboardingLink, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let account = sqlx::query!(
    r#"
      select a.stripe_id
      from accounts a
      where 
        a.id = $1 and
        exists (
          select 1
          from store_members sm
          where sm.store_id = a.store_id and sm.user_id = $2
        )
    "#,
    &account_id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The account '{account_id}' could not be found"
    )
  })?;

  let create_account_link_params = CreateAccountLinkParams::new(
    &account.stripe_id,
    AccountLinkType::AccountOnboarding,
    // TODO: decide this later
    &state.config.website_base_url,
    // TODO: decide this later
    &state.config.website_base_url,
  );

  let account_link = state
    .stripe
    .account_links
    .create(create_account_link_params)
    .await
    .map_err(|_| failure!())?;

  Ok(OnboardingLink {
    url: account_link.url,
    expires_at: DateTime::from_timestamp(account_link.expires_at, 0)
      .ok_or_else(|| failure!())?,
    created_at: DateTime::from_timestamp(account_link.created, 0)
      .ok_or_else(|| failure!())?,
  })
}
