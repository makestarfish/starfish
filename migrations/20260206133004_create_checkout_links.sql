create table checkout_links (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  client_secret text unique not null,
  label text,
  success_url text,
  created_at timestamptz not null default now(),
  modified_at timestamptz,
  
  foreign key (store_id) references stores (id)
)
