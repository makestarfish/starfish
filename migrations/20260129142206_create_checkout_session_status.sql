create type checkout_session_status as enum (
  'open',
  'expired',
  'confirmed',
  'failed',
  'succeeded'
)
