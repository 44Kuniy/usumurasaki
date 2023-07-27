use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{ChannelId, ToSqlValue, ToSqlValues};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct VideoId(String);
async_graphql::scalar!(VideoId);

impl ToString for VideoId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
impl VideoId {
    pub fn inner(self) -> String {
        self.0
    }
}
#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct Video {
    pub id: VideoId,
    pub channel_id: ChannelId,
    pub embed_code: String,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewVideo {
    pub id: VideoId,
    pub channel_id: ChannelId,
    pub embed_code: String,
}

impl ToSqlValue for NewVideo {
    fn into_sql_value(self) -> String {
        format!(
            "({}, {}, {})",
            self.id.0,
            self.channel_id.inner(),
            self.embed_code
        )
    }
}

impl ToSqlValues<NewVideo> for Vec<NewVideo> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
