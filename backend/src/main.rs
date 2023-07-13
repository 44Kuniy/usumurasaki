use sqlx::migrate::Migrator;

use sqlx::postgres::PgPoolOptions;

static MIGRATOR: Migrator = sqlx::migrate!("./../migrations");

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url())
        .await?;

    MIGRATOR.run(&pool).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    println!("{}", row.0);
    assert_eq!(row.0, 150);

    Ok(())
}

fn database_url() -> String {
    // postgres://postgres:postgres@localhost:5434/usu
    let user = "postgres";
    let password = "postgres";
    let host = "localhost";
    let port = "5434";
    let db = "usu";
    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db)
}
