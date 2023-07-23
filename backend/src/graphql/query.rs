use crate::models::*;
use crate::repository::UserRepo;
use async_graphql::Object;
use async_graphql::Result as GqlResult;
use sqlx::Pool;
use sqlx::Postgres;
pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self, _ctx: &async_graphql::Context<'_>) -> &'static str {
        "partner"
    }

    async fn channels(&self, _ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Channel>> {
        let pool = _ctx.data::<Pool<Postgres>>()?;
        let channels = sqlx::query_as::<_, Channel>("select * from channels")
            .fetch_all(pool)
            .await?;
        Ok(channels)
    }

    async fn users(&self, _ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<User>> {
        println!("users QUERY");
        let pool = _ctx.data::<Pool<Postgres>>()?;
        let users = UserRepo::new(pool).all().await?;
        Ok(users)
    }
}
