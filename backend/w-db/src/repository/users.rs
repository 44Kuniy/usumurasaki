use sqlx::PgPool;

use w_models::{NewPartner, Partner, SqlxResult, ToSqlValues};

pub struct PartnerRepo<'a>(pub &'a PgPool);

impl<'a> PartnerRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self(pool)
    }
}

impl<'a> PartnerRepo<'a> {
    pub async fn create(&self, new: Vec<NewPartner>) -> SqlxResult<Vec<Partner>> {
        sqlx::query_as::<_, Partner>(&format!(
            "INSERT INTO partners (id)
                VALUES {:#?};",
            new.into_sql_values()
        ))
        .fetch_all(self.0)
        .await
    }

    pub async fn create_or_update(&self, new: Vec<NewPartner>) -> SqlxResult<Vec<Partner>> {
        sqlx::query_as::<_, Partner>(&format!(
            "INSERT INTO partners (id)
                VALUES {:#?}
                ON CONFLICT (mail) 
                DO UPDATE 
                    SET name = EXCLUDED.name
                RETURNING (id, name, mail, inserted_at, updated_at)
                ;",
            new.into_sql_values(),
        ))
        .fetch_all(self.0)
        .await
    }

    pub async fn all(&self) -> SqlxResult<Vec<Partner>> {
        sqlx::query_as::<_, Partner>("select * from partners;")
            .fetch_all(self.0)
            .await
    }
}
