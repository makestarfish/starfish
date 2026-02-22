use async_graphql::{EmptySubscription, Schema, dataloader::DataLoader};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, extract::State, http::HeaderMap, routing::post};
use starfish::{
  context::RequestContext,
  dataloader::{PriceLoader, ProductLoader, StandardLoader},
  mutations::Mutation,
  queries::Query,
  routes::stripe_webhook_handler,
  state::SharedState,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn graphql(
  State(schema): State<Schema<Query, Mutation, EmptySubscription>>,
  headers: HeaderMap,
  request: GraphQLRequest,
) -> GraphQLResponse {
  let mut request = request.into_inner();

  let state = schema.data::<SharedState>().unwrap();

  let request_context =
    RequestContext::from_headers(headers, &state.config.jwt_secret);

  request = request.data(request_context);

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
    .data(DataLoader::new(
      StandardLoader::new(state.db.clone(), state.config.clone()),
      tokio::spawn,
    ))
    .data(DataLoader::new(
      PriceLoader::new(state.db.clone()),
      tokio::spawn,
    ))
    .data(DataLoader::new(
      ProductLoader::new(state.db.clone()),
      tokio::spawn,
    ))
    .finish();

  let router = Router::new()
    .route("/", post(graphql))
    .with_state(schema)
    .route("/webhooks/stripe", post(stripe_webhook_handler::handle))
    .with_state(state.clone())
    .layer(CorsLayer::very_permissive());

  axum::serve(listener, router).await.unwrap()
}
