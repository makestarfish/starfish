alter table checkout_sessions
  add column product_id uuid not null,
  add constraint checkout_sessions_product_id_fkey
    foreign key (product_id) references products (id)