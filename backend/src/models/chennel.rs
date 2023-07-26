use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use itertools::Itertools;

use super::{ToSqlValue, ToSqlValues};

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct Channel {
    pub id: String,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewChannel {
    pub id: String,
}

impl ToSqlValue for NewChannel {
    fn into_sql_value(self) -> String {
        format!("({})", self.id)
    }
}

impl ToSqlValues<NewChannel> for Vec<NewChannel> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
