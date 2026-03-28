create table users (
  id uuid primary key default uuidv7(),
  email text unique not null,
  name text,
  avatar_url text,
  created_at timestamptz not null default now(),
  modified_at timestamptz
)