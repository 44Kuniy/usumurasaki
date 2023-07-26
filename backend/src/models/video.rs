use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use itertools::Itertools;

use super::{ToSqlValue, ToSqlValues};

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct Video {
    pub id: String,
    pub channel_id: String,
    pub embed_code: String,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewVideo {
    pub id: String,
    pub channel_id: String,
    pub embed_code: String,
}

impl ToSqlValue for NewVideo {
    fn into_sql_value(self) -> String {
        format!("({}, {}, {})", self.id, self.channel_id, self.embed_code)
    }
}

impl ToSqlValues<NewVideo> for Vec<NewVideo> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
