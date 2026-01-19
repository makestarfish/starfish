create table refresh_tokens (
  id uuid primary key default uuidv7(),
  session_id uuid not null,
  revoked boolean not null default false,

  foreign key (session_id) references sessions (id)
)