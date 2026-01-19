create table sessions (
  id uuid primary key default uuidv7(),
  user_id uuid not null,
  revoked boolean not null default false,
  last_seen_at timestamptz not null default now(),

  foreign key (user_id) references users (id)
)