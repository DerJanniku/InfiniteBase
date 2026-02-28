use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn connect(url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("Failed to connect to Postgres")
}
