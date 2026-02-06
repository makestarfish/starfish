create table checkout_link_products (
  id uuid primary key default uuidv7(),
  checkout_link_id uuid not null,
  product_id uuid not null,

  foreign key (checkout_link_id) references checkout_links (id),
  foreign key (product_id) references products (id),
  unique (checkout_link_id, product_id)
)