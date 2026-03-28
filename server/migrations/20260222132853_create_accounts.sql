create table accounts (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  stripe_id varchar(255) unique not null,

  foreign key (store_id) references stores (id)
)