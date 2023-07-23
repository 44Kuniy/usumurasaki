use async_graphql::SimpleObject;

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct Channel {
    pub id: i64,
}
