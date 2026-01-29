use crate::{
  entities::{Price, ProductId},
  failure::Failure,
};
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, types::Json};
use uuid::Uuid;

pub struct DataLoader {
  db: PgPool,
}

impl DataLoader {
  pub fn new(db: PgPool) -> Self {
    Self { db }
  }
}

impl Loader<ProductId> for DataLoader {
  type Value = Vec<Price>;
  type Error = Failure;

  async fn load(
    &self,
    keys: &[ProductId],
  ) -> Result<std::collections::HashMap<ProductId, Self::Value>, Self::Error>
  {
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
