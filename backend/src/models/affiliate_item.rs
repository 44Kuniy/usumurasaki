use async_graphql::SimpleObject;
use chrono::NaiveDateTime;

use itertools::Itertools;

use super::{ToSqlValues, ToSqlValuesWithi64Arg};

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct AffiliateItem {
    pub id: i64,
    pub amazon_url: String,
    pub is_valid: bool,
    pub partner_id: i64,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewAffiliateItem {
    pub amazon_url: String,
    pub is_valid: bool,
    pub partner_id: i64,
}

impl NewAffiliateItem {
    pub fn into_sql_value(self) -> String {
        format!(
            "(default, {}, {}, {})",
            self.amazon_url, self.is_valid, self.partner_id
        )
    }
}

impl ToSqlValues<NewAffiliateItem> for Vec<NewAffiliateItem> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VideoAffilicateItemsBelonging {
    pub video_id: String,
    pub affiliate_item_id: i64,
}
impl VideoAffilicateItemsBelonging {
    pub fn into_sql_value(self) -> String {
        format!("({}, {})", self.video_id, self.affiliate_item_id,)
    }
}
impl ToSqlValues<VideoAffilicateItemsBelonging> for Vec<VideoAffilicateItemsBelonging> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChannelAffilicateItemsBelonging {
    pub channel_id: String,
    pub affiliate_item_id: i64,
}
impl ChannelAffilicateItemsBelonging {
    pub fn into_sql_value(self) -> String {
        format!("({}, {})", self.channel_id, self.affiliate_item_id)
    }
}
impl ToSqlValues<ChannelAffilicateItemsBelonging> for Vec<ChannelAffilicateItemsBelonging> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
