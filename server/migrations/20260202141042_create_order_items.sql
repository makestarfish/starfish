create table order_items (
  id uuid primary key default uuidv7(),
  order_id uuid not null,
  product_price_id uuid not null,
  label text not null,
  amount bigint not null,
  tax_amount bigint not null,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (order_id) references orders (id),
  foreign key (product_price_id) references prices (id)
)
