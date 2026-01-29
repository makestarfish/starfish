create table checkout_sessions (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  customer_id uuid,
  status checkout_session_status not null default 'open',
  client_secret text not null,
  amount bigint not null,
  discount_amount bigint not null default 0,
  tax_amount bigint,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (store_id) references stores (id),
  foreign key (customer_id) references customers (id)
)