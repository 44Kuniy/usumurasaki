use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{ToSqlValue, ToSqlValues};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ChannelId(String);
async_graphql::scalar!(ChannelId);

impl ToString for ChannelId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
impl ChannelId {
    pub fn inner(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct Channel {
    pub id: ChannelId,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewChannel {
    pub id: ChannelId,
}

impl ToSqlValue for NewChannel {
    fn into_sql_value(self) -> String {
        format!("({})", self.id.0)
    }
}

impl ToSqlValues<NewChannel> for Vec<NewChannel> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
