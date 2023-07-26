use sqlx::PgPool;

use crate::models::{AffiliateItem, NewAffiliateItem, SqlxResult, ToSqlValues};

pub struct AffiliateItemRepo<'a>(pub &'a PgPool);

impl<'a> AffiliateItemRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }
}

impl<'a> AffiliateItemRepo<'a> {
    pub async fn create(&self, new: Vec<NewAffiliateItem>) -> SqlxResult<Vec<AffiliateItem>> {
        sqlx::query_as::<_, AffiliateItem>(&format!(
            "INSERT INTO affiliate_items (id, amazon_url, is_valid, partner_id)
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
