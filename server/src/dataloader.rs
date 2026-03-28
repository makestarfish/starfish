use crate::{
  config::Config,
  entities::{
    BillingReason, CheckoutSession, CheckoutSessionId, CheckoutSessionStatus,
    Customer, CustomerId, Order, OrderId, OrderItem, OrderStatus, Price,
    Product, ProductId, Store, StoreId, StoreStatus, Transaction,
    TransactionId,
  },
  failure::Failure,
};
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, types::Json};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StandardLoader {
  db: PgPool,
  config: Config,
}

impl StandardLoader {
  pub fn new(db: PgPool, config: Config) -> Self {
    Self { db, config }
  }
}

impl Loader<StoreId> for StandardLoader {
  type Value = Store;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[StoreId],
  ) -> Result<HashMap<StoreId, Self::Value>, Self::Error> {
    sqlx::query_as!(
      Store,
      r#"
          select 
            id, 
            slug, 
            name, 
            status as "status: StoreStatus",
            email, 
            website, 
            avatar_url, 
            created_at, 
            modified_at
          from stores
          where id = any($1)
        "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>(),
    )
    .fetch_all(&self.db)
    .await
    .map(|stores| {
      stores
        .into_iter()
        .map(|store| (store.id.to_owned(), store))
        .collect()
    })
    .map_err(|_| failure!())
  }
}

impl Loader<ProductId> for StandardLoader {
  type Value = Product;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[ProductId],
  ) -> Result<HashMap<ProductId, Self::Value>, Self::Error> {
    sqlx::query_as!(
      Product,
      r#"
        select id, store_id, name, description, archived, created_at, modified_at
        from products
        where id = any($1)
      "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>()
    )
    .fetch_all(&self.db)
    .await
    .map(|products| {
      products
        .into_iter()
        .map(|product| (product.id.to_owned(), product))
        .collect()
    })
    .map_err(|_| failure!())
  }
}

impl Loader<CheckoutSessionId> for StandardLoader {
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
            success_url,
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

impl Loader<CustomerId> for StandardLoader {
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

impl Loader<OrderId> for StandardLoader {
  type Value = Order;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[OrderId],
  ) -> Result<HashMap<OrderId, Self::Value>, Self::Error> {
    sqlx::query_as!(
      Order,
      r#"
        select 
          id,
          store_id,
          customer_id,
          checkout_session_id as "checkout_session_id: CheckoutSessionId",
          status as "status: OrderStatus",
          subtotal_amount,
          discount_amount,
          (subtotal_amount - discount_amount) as "net_amount!",
          tax_amount,
          (subtotal_amount - discount_amount + tax_amount) as "total_amount!",
          platform_fee_amount,
          billing_reason as "billing_reason: BillingReason",
          created_at,
          modified_at
        from orders
        where id = any($1)
      "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>()
    )
    .fetch_all(&self.db)
    .await
    .map(|orders| {
      orders
        .into_iter()
        .map(|order| (order.id.to_owned(), order))
        .collect()
    })
    .map_err(|_| failure!())
  }
}

pub struct PriceLoader {
  db: PgPool,
}

impl PriceLoader {
  pub fn new(db: PgPool) -> Self {
    Self { db }
  }
}

impl Loader<ProductId> for PriceLoader {
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

pub struct ProductLoader {
  db: PgPool,
}

impl ProductLoader {
  pub fn new(db: PgPool) -> Self {
    Self { db }
  }
}

impl Loader<CheckoutSessionId> for ProductLoader {
  type Value = Vec<Product>;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[CheckoutSessionId],
  ) -> Result<HashMap<CheckoutSessionId, Self::Value>, Self::Error> {
    sqlx::query!(
      r#"
        select 
          checkout_session_id,
          jsonb_agg(
            jsonb_build_object(
              'id', products.id,
              'store_id', products.store_id,
              'name', products.name,
              'description', products.description,
              'archived', products.archived,
              'created_at', products.created_at,
              'modified_at', products.modified_at
            )
          ) as "products: Json<Vec<Product>>"
        from checkout_session_products
        join products on checkout_session_products.product_id = products.id
        where checkout_session_id = any($1)
        group by checkout_session_id
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
            CheckoutSessionId(group.checkout_session_id),
            group
              .products
              .map(|products| products.0)
              .unwrap_or_default(),
          )
        })
        .collect()
    })
    .map_err(|_| failure!())
  }
}

pub struct OrderItemLoader {
  db: PgPool,
}

impl OrderItemLoader {
  pub fn new(db: PgPool) -> Self {
    Self { db }
  }
}

impl Loader<OrderId> for OrderItemLoader {
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

pub struct IncurredTransactionLoader {
  db: PgPool,
}

impl IncurredTransactionLoader {
  pub fn new(db: PgPool) -> Self {
    Self { db }
  }
}

impl Loader<TransactionId> for IncurredTransactionLoader {
  type Value = Vec<Transaction>;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[TransactionId],
  ) -> Result<HashMap<TransactionId, Self::Value>, Self::Error> {
    sqlx::query_as!(
      Transaction,
      r#"
          select 
            id, 
            incurred_by as "incurred_by: TransactionId",
            account_id,
            order_id,
            amount, 
            incurred_amount,
            (amount - incurred_amount) as "net_amount!",
            created_at,
            modified_at
          from transactions
          where incurred_by = any($1)
          order by id desc
        "#,
      &keys.iter().map(|id| id.0).collect::<Vec<Uuid>>()
    )
    .fetch_all(&self.db)
    .await
    .map(|transactions| {
      transactions.into_iter().fold(
        HashMap::<TransactionId, Vec<Transaction>>::new(),
        |mut acc, transaction| {
          acc
            .entry(transaction.incurred_by.clone().unwrap())
            .or_default()
            .push(transaction);

          acc
        },
      )
    })
    .map_err(|_| failure!())
  }
}
