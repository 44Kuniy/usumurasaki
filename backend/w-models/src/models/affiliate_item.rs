use async_graphql::SimpleObject;
use chrono::NaiveDateTime;

use itertools::Itertools;

use super::{ChannelId, ToSqlValues, ToSqlValuesWithi64Arg, VideoId};

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
pub struct NewVideoAffiliateItemsBelonging {
    pub video_id: VideoId,
    pub affiliate_item_id: i64,
}
impl NewVideoAffiliateItemsBelonging {
    pub fn into_sql_value(self) -> String {
        format!(
            "({}, {}, default,default)",
            self.video_id.to_string(),
            self.affiliate_item_id
        )
    }
}

impl ToSqlValues<NewVideoAffiliateItemsBelonging> for Vec<NewVideoAffiliateItemsBelonging> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VideoAffiliateItemsBelonging {
    pub video_id: String,
    pub affiliate_item_id: i64,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl VideoAffiliateItemsBelonging {
    pub fn into_sql_value(self) -> String {
        format!("({}, {})", self.video_id, self.affiliate_item_id,)
    }
}
impl ToSqlValues<VideoAffiliateItemsBelonging> for Vec<VideoAffiliateItemsBelonging> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewChannelAffiliateItemsBelonging {
    pub channel_id: ChannelId,
    pub affiliate_item_id: i64,
}
impl NewChannelAffiliateItemsBelonging {
    pub fn into_sql_value(self) -> String {
        format!(
            "({}, {}, default,default)",
            self.channel_id.inner(),
            self.affiliate_item_id
        )
    }
}

impl ToSqlValues<NewChannelAffiliateItemsBelonging> for Vec<NewChannelAffiliateItemsBelonging> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChannelAffiliateItemsBelonging {
    pub channel_id: String,
    pub affiliate_item_id: i64,
}
impl ChannelAffiliateItemsBelonging {
    pub fn into_sql_value(self) -> String {
        format!("({}, {})", self.channel_id, self.affiliate_item_id)
    }
}
impl ToSqlValues<ChannelAffiliateItemsBelonging> for Vec<ChannelAffiliateItemsBelonging> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
