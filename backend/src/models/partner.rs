use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use itertools::Itertools;

use super::{ToSqlValue, ToSqlValues};

#[derive(Debug, Clone, SimpleObject, sqlx::FromRow)]
pub struct Partner {
    pub id: i64,
    pub name: String,
    pub mail: String,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewPartner {
    pub name: String,
    pub mail: String,
}

impl ToSqlValue for NewPartner {
    fn into_sql_value(self) -> String {
        format!("({}, {})", self.name, self.mail)
    }
}

impl ToSqlValues<NewPartner> for Vec<NewPartner> {
    fn into_sql_values(self) -> String {
        self.into_iter().map(|n| n.into_sql_value()).join(",")
    }
}
