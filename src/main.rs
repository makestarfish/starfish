use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, extract::State, routing::post};
use starfish::{mutations::Mutation, queries::Query, state::SharedState};
use tokio::net::TcpListener;

async fn graphql(
  State(schema): State<Schema<Query, Mutation, EmptySubscription>>,
  request: GraphQLRequest,
) -> GraphQLResponse {
  let request = request.into_inner();

  schema.execute(request).await.into()
}

#[tokio::main]
async fn main() {
  let state = SharedState::from_env().await;

  let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
    .await
    .unwrap();

  let schema = Schema::build(Query, Mutation, EmptySubscription)
    .data(state.clone())
    .finish();

  let router = Router::new().route("/", post(graphql)).with_state(schema);

  axum::serve(listener, router).await.unwrap()
}
