create table transactions (
  id uuid primary key default uuidv7(),
  account_id uuid not null,
  amount bigint not null,
  fee_amount bigint not null,
  created_at timestamptz not null default now(),

  foreign key (account_id) references accounts (id)
)
