create table store_access_tokens (
  id uuid primary key default uuidv7(),
  store_id uuid not null,
  
  foreign key (store_id) references stores (id)
)