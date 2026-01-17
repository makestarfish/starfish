use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
  async fn message(&self) -> String {
    String::default()
  }
}
