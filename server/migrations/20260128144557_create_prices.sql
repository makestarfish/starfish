create table prices (
  id uuid primary key default uuidv7(),
  product_id uuid not null,
  amount bigint not null,
  archived boolean not null default false,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (product_id) references products (id)
)
