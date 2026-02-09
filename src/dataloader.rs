use crate::{
  config::Config,
  entities::{
    CheckoutSession, CheckoutSessionId, CheckoutSessionStatus, Customer,
    CustomerId, OrderId, OrderItem, Price, ProductId,
  },
  failure::Failure,
};
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, types::Json};
use std::collections::HashMap;
use uuid::Uuid;

pub struct DataLoader {
  db: PgPool,
  config: Config,
}

impl DataLoader {
  pub fn new(db: PgPool, config: Config) -> Self {
    Self { db, config }
  }
}

impl Loader<ProductId> for DataLoader {
  type Value = Vec<Price>;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[ProductId],
  ) -> Result<HashMap<ProductId, Self::Value>, Self::Error> {
    sqlx::query!(
      r#"
        select 
          product_id, 
          jsonb_agg(
            jsonb_build_object(
              'id', id,
              'product_id', product_id,
              'amount', amount,
              'archived', archived,
              'created_at', created_at,
              'modified_at', modified_at
            )
          ) as "prices: Json<Vec<Price>>"
        from prices
        where product_id = any($1)
        group by product_id
      "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>()
    )
    .fetch_all(&self.db)
    .await
    .map(|groups| {
      groups
        .into_iter()
        .map(|group| {
          (
            ProductId(group.product_id),
            group.prices.map(|prices| prices.0).unwrap_or_default(),
          )
        })
        .collect()
    })
    .map_err(|_| failure!())
  }
}

impl Loader<CheckoutSessionId> for DataLoader {
  type Value = CheckoutSession;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[CheckoutSessionId],
  ) -> Result<HashMap<CheckoutSessionId, Self::Value>, Self::Error> {
    sqlx::query_as!(
        CheckoutSession,
        r#"
          select 
            id, 
            store_id, 
            product_id,
            customer_id as "customer_id: CustomerId",
            customer_email,
            client_secret,
            status as "status: CheckoutSessionStatus",
            rtrim($2, '/') || '/checkout/' || client_secret as "url!",
            amount,
            discount_amount,
            tax_amount,
            (amount - discount_amount) as "net_amount!",
            (amount - discount_amount + coalesce(tax_amount, 0)) as "total_amount!",
            created_at,
            modified_at
          from checkout_sessions
          where id = any($1)
        "#,
        &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>(),
        &self.config.website_base_url,
      )
      .fetch_all(&self.db)
      .await
      .map(|checkout_sessions| {
        checkout_sessions
          .into_iter()
          .map(|checkout_session| (checkout_session.id.to_owned(), checkout_session))
          .collect()
      })
      .map_err(|_| failure!())
  }
}

impl Loader<CustomerId> for DataLoader {
  type Value = Customer;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[CustomerId],
  ) -> Result<HashMap<CustomerId, Self::Value>, Self::Error> {
    sqlx::query_as!(
      Customer,
      r#"
        select id, store_id, email, name, avatar_url, created_at, modified_at
        from customers
        where id = any($1)
      "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>()
    )
    .fetch_all(&self.db)
    .await
    .map(|customers| {
      customers
        .into_iter()
        .map(|customer| (customer.id.to_owned(), customer))
        .collect()
    })
    .map_err(|_| failure!())
  }
}

impl Loader<OrderId> for DataLoader {
  type Value = Vec<OrderItem>;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[OrderId],
  ) -> Result<HashMap<OrderId, Self::Value>, Self::Error> {
    sqlx::query!(
      r#"
        select
          order_id,
          jsonb_agg(
            jsonb_build_object(
              'id', id,
              'order_id', order_id,
              'product_price_id', product_price_id,
              'label', label,
              'amount', amount,
              'tax_amount', tax_amount,
              'created_at', created_at,
              'modified_at', modified_at
            )
          ) as "items: Json<Vec<OrderItem>>"
        from order_items
        where order_id = any($1)
        group by order_id
      "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>()
    )
    .fetch_all(&self.db)
    .await
    .map(|groups| {
      groups
        .into_iter()
        .map(|group| {
          (
            OrderId(group.order_id),
            group.items.map(|item| item.0).unwrap_or_default(),
          )
        })
        .collect()
    })
    .map_err(|_| failure!())
  }
}
