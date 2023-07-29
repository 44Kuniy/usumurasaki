pub mod repository;

pub use crate::repository::*;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool};

fn database_url() -> String {
    // postgres://postgres:postgres@localhost:5434/usu
    let user = "postgres";
    let password = "postgres";
    let host = "localhost";
    let port = "5434";
    let db = "usu";
    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db)
}

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url())
        .await?;
    Ok(pool)
}
