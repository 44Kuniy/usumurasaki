use actix_web::web::Data;
use async_graphql::Object;
use async_graphql::Result as GqlResult;
use sqlx::Pool;
use sqlx::Postgres;
use w_db::repository::PartnerRepo;
use w_models::*;

use crate::server_context::ContextData;
pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self, _ctx: &async_graphql::Context<'_>) -> &'static str {
        "partner"
    }

    async fn channels(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Channel>> {
        println!("call channels");
        let data = ctx.data::<Data<ContextData>>()?;
        let pool = &data.pool;
        let config = &data.config;
        println!("config ❓: {:#?}", config);

        let channels = sqlx::query_as::<_, Channel>("select * from channels")
            .fetch_all(pool)
            .await?;
        Ok(channels)
    }

    async fn partners(&self, _ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Partner>> {
        println!("partners QUERY");
        let pool = _ctx.data::<Pool<Postgres>>()?;
        let users = PartnerRepo::new(pool).all().await?;
        Ok(users)
    }
}
