create table one_time_tokens (
  id uuid primary key default uuidv7(),
  email text not null,
  code_hash text not null,
  expires_at timestamptz not null,
  was_used boolean not null default false
)