use sqlx::PgPool;

use crate::User;

pub struct UserRepo<'a>(pub &'a PgPool);

impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }
}

impl<'a> UserRepo<'a> {
    pub async fn all(&self) -> sqlx::Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("select * from users")
            .fetch_all(self.0)
            .await
    }
}
