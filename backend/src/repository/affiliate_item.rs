use itertools::multiunzip;
use sqlx::PgPool;

use crate::models::{
    AffiliateItem, ChannelId, NewAffiliateItem, NewChannelAffiliateItemsBelonging,
    NewVideoAffiliateItemsBelonging, SqlxResult, ToSqlValues, VideoId,
};

pub struct AffiliateItemRepo<'a>(pub &'a PgPool);

impl<'a> AffiliateItemRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }
}

impl<'a> AffiliateItemRepo<'a> {
    pub async fn create(
        &self,
        new: Vec<(NewAffiliateItem, VideoId, ChannelId)>,
    ) -> SqlxResult<Vec<AffiliateItem>> {
        let mut new_v_a_belonging = vec![];
        let mut new_c_a_belonging = vec![];

        for (item, video_id, channel_id) in new.into_iter() {
            let created = sqlx::query_as::<_, AffiliateItem>(&format!(
                "INSERT INTO affiliate_items (id, amazon_url, is_valid, partner_id)
                VALUES {:#?};",
                item.into_sql_value()
            ))
            .fetch_one(self.0)
            .await?;
            new_v_a_belonging.push(NewVideoAffiliateItemsBelonging {
                video_id: video_id,
                affiliate_item_id: created.id,
            });
            new_c_a_belonging.push(NewChannelAffiliateItemsBelonging {
                channel_id: channel_id,
                affiliate_item_id: created.id,
            });
        }
        todo!()
    }

    pub async fn create_video_affiliate_items_belongings(
        &self,
        new: Vec<NewVideoAffiliateItemsBelonging>,
    ) -> SqlxResult<Vec<AffiliateItem>> {
        sqlx::query_as::<_, AffiliateItem>(&format!(
            "INSERT INTO video_affiliate_item_belongings (video_id, affiliate_item_id, inserted_at, updated_at)
                VALUES {:#?};",
            new.into_sql_values()
        ))
        .fetch_all(self.0)
        .await
    }

    pub async fn create_channel_affiliate_item_belongings(
        &self,
        new: Vec<NewVideoAffiliateItemsBelonging>,
    ) -> SqlxResult<Vec<AffiliateItem>> {
        sqlx::query_as::<_, AffiliateItem>(&format!(
            "INSERT INTO channel_affiliate_item_belongings (channel_id, affiliate_item_id, inserted_at, updated_at)
                VALUES {:#?};",
            new.into_sql_values()
        ))
        .fetch_all(self.0)
        .await
    }

    pub async fn all(&self) -> SqlxResult<Vec<AffiliateItem>> {
        sqlx::query_as::<_, AffiliateItem>("select * from affiliate_items")
            .fetch_all(self.0)
            .await
    }
}
