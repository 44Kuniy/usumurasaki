use itertools::Itertools;
use sqlx::PgPool;

use crate::models::{Channel, NewChannel, SqlxResult, ToSqlValue, ToSqlValues};

pub struct ChannelRepo<'a>(pub &'a PgPool);

impl<'a> ChannelRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }
}

impl<'a> ChannelRepo<'a> {
    pub async fn create(&self, new: Vec<NewChannel>) -> SqlxResult<Vec<Channel>> {
        sqlx::query_as::<_, Channel>(&format!(
            "INSERT INTO channels (id)
                VALUES {:#?}",
            new.into_sql_values()
        ))
        .fetch_all(self.0)
        .await
    }

    pub async fn all(&self) -> SqlxResult<Vec<Channel>> {
        sqlx::query_as::<_, Channel>("select * from channels")
            .fetch_all(self.0)
            .await
    }
}
