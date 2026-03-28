create table store_members (
  id uuid primary key default uuidv7(),
  user_id uuid not null,
  store_id uuid not null,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (user_id) references users (id),
  foreign key (store_id) references stores (id)
)