alter table checkout_sessions
  add column stripe_id varchar(255) unique not null