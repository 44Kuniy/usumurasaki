use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self, _ctx: &async_graphql::Context<'_>) -> &'static str {
        "partner"
    }
}
