use crate::{
  event_handlers::create_order_from_checkout_session,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use axum::{extract::State, http::HeaderMap, response::IntoResponse};

pub async fn handle(
  State(state): State<SharedState>,
  headers: HeaderMap,
  body: String,
) -> Result<impl IntoResponse, Failure> {
  let signature = headers
    .get("stripe-signature")
    .and_then(|v| v.to_str().ok())
    .ok_or_else(|| {
      failure!(
        FailureReason::UNAUTHORIZED,
        "The 'stripe-signature' header is missing"
      )
    })?;

  let event = state
    .stripe
    .webhooks
    .construct_event(
      &body,
      signature,
      &state.config.stripe_webhook_signing_secret,
    )
    .await
    .map_err(|_| {
      failure!(FailureReason::UNAUTHORIZED, "Failed to construct event")
    })?;

  create_order_from_checkout_session::handle(&state, event).await
}
