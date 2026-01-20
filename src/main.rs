use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, extract::State, http::HeaderMap, routing::post};
use starfish::{
  context::RequestContext, mutations::Mutation, queries::Query,
  state::SharedState,
};
use tokio::net::TcpListener;

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
    .finish();

  let router = Router::new().route("/", post(graphql)).with_state(schema);

  axum::serve(listener, router).await.unwrap()
}
