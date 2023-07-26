use sqlx::PgPool;

use crate::models::{NewVideo, SqlxResult, ToSqlValues, Video};

pub struct VideoRepo<'a>(pub &'a PgPool);

impl<'a> VideoRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }
}

impl<'a> VideoRepo<'a> {
    pub async fn create(&self, new: Vec<NewVideo>) -> SqlxResult<Vec<Video>> {
        sqlx::query_as::<_, Video>(&format!(
            "INSERT INTO videos (id)
                VALUES {:#?};",
            new.into_sql_values()
        ))
        .fetch_all(self.0)
        .await
    }

    pub async fn all(&self) -> SqlxResult<Vec<Video>> {
        sqlx::query_as::<_, Video>("select * from videos")
            .fetch_all(self.0)
            .await
    }
}
