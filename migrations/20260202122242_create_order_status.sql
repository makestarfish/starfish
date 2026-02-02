create type order_status as enum (
  'pending',
  'paid',
  'refunded', 
  'partially_refunded'
)