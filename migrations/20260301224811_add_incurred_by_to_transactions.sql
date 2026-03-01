alter table transactions
  add column incurred_by uuid,
  add constraint transactions_incurred_by_fkey
    foreign key (incurred_by) references transactions (id)