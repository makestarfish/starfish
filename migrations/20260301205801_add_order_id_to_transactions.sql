alter table transactions
  add column order_id uuid not null,
  add constraint transactions_order_id_fkey
    foreign key (order_id) references orders (id)