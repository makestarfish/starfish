create type billing_reason as enum (
  'purchase',
  'subscription_creation',
  'subscription_renewal',
  'subscription_update'
)
