create table checkout_session_products (
  id uuid primary key default uuidv7(),
  checkout_session_id uuid not null,
  product_id uuid not null,

  foreign key (checkout_session_id) references checkout_sessions (id),
  foreign key (product_id) references products (id),
  unique (checkout_session_id, product_id)
)