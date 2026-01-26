create table customers (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  email text not null,
  name text,
  deleted_at timestamptz,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (store_id) references stores (id)
)