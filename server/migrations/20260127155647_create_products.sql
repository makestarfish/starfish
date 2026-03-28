create table products (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  name text not null,
  description text,
  archived boolean not null default false,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (store_id) references stores (id)
)