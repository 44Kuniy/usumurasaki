use async_graphql::SimpleObject;

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}
