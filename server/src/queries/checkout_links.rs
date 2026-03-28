use crate::{
  context::RequestContext,
  entities::{CheckoutLink, CheckoutLinkConnection, CheckoutLinkEdge},
  failure::{Failure, FailureReason},
  state::SharedState,
};
use uuid::Uuid;

pub async fn resolve(
  state: &SharedState,
  context: &RequestContext,
  store_id: Uuid,
  _first: Option<i64>,
  after: Option<Uuid>,
  _last: Option<i64>,
  before: Option<Uuid>,
) -> Result<CheckoutLinkConnection, Failure> {
  let user_id = context
    .user_id
    .ok_or_else(|| failure!(FailureReason::UNAUTHORIZED))?;

  let store = sqlx::query!(
    r#"
      select exists (
        select 1
        from store_members sm
        where sm.store_id = s.id and user_id = $2
      ) as member_exists
      from stores s
      where s.id = $1
    "#,
    &store_id,
    &user_id,
  )
  .fetch_optional(&state.db)
  .await
  .map_err(|_| failure!())?
  .ok_or_else(|| {
    failure!(
      FailureReason::NOT_FOUND,
      "The store '{store_id}' could not be found"
    )
  })?;

  if !store.member_exists.unwrap_or_default() {
    bail!(
      FailureReason::FORBIDDEN,
      "You are not a member of this store"
    )
  }

  let checkout_links = sqlx::query_as!(
    CheckoutLink,
    r#"
      select 
        id, 
        store_id, 
        client_secret,
        label, 
        success_url,
        rtrim($4, '/') || '/links/' || client_secret as "url!",
        created_at,
        modified_at
      from checkout_links
      where 
        store_id = $1 and
        (id < $2 or $2 is null) and 
        (id > $3 or $3 is null)
    "#,
    &store_id,
    after,
    before,
    &state.config.website_base_url,
  )
  .fetch_all(&state.db)
  .await
  .map_err(|_| failure!())?;

  Ok(CheckoutLinkConnection {
    edges: checkout_links
      .iter()
      .map(|checkout_link| CheckoutLinkEdge {
        cursor: checkout_link.id.to_owned(),
        node: checkout_link.to_owned(),
      })
      .collect(),
    nodes: checkout_links,
  })
}
