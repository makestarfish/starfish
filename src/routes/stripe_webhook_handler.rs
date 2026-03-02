use crate::{
  event_handlers::create_order_from_checkout_session,
  failure::{Failure, FailureReason},
  state::SharedState,
};
use axum::{extract::State, http::HeaderMap};
use starfish_stripe::types::Event;

pub async fn handle(
  State(state): State<SharedState>,
  headers: HeaderMap,
  body: String,
) -> Result<(), Failure> {
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

  match event {
    Event::PaymentIntentSucceeded { data, .. } => {
      create_order_from_checkout_session::handle(&state, data.object).await
    }
    _ => Ok(()),
  }
}
