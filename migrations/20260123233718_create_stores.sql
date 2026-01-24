create table stores (
  id uuid primary key default uuidv7(),
  slug text unique not null,
  name text not null,
  email text,
  website text,
  avatar_url text,
  created_at timestamptz not null default now(),
  modified_at timestamptz
)