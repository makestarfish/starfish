#[tokio::main]
async fn main() -> Result<(), sqlx::migrate::MigrateError> {
  sqlx::migrate!()
    .run(
      &sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(
          std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for migrations")
            .as_str(),
        )
        .await
        .unwrap(),
    )
    .await
}
