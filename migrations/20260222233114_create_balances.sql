create table balances (
  account_id uuid unique not null,
  amount bigint not null,

  foreign key (account_id) references accounts (id)
)