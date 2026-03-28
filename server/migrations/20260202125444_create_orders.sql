create table orders (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  customer_id uuid not null,
  checkout_session_id uuid,
  status order_status not null default 'pending',
  subtotal_amount bigint not null,
  discount_amount bigint not null,
  tax_amount bigint not null,
  platform_fee_amount bigint not null,
  billing_reason billing_reason not null default 'purchase',
  created_at timestamptz not null default now(),
  modified_at timestamptz,
  
  foreign key (store_id) references stores (id),
  foreign key (customer_id) references customers (id),
  foreign key (checkout_session_id) references checkout_sessions (id)
)