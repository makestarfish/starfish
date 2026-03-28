create table store_invites (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  inviter_id uuid not null,
  invitee_id uuid,
  email text not null,
  accepted_at timestamptz,
  revoked_at timestamptz,
  created_at timestamptz not null default now(),
  modified_at timestamptz,

  foreign key (store_id) references stores (id),
  foreign key (inviter_id) references users (id),
  foreign key (invitee_id) references users (id)
)