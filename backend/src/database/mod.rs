use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use qdrant_client::qdrant::QdrantClient;
use std::env;
use anyhow::Result;

pub async fn init() -> Result<Pool<Postgres>> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://infinitebase:localhost_only@localhost:5432/infinitebase".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    Ok(pool)
}

pub async fn init_qdrant() -> Result<QdrantClient> {
    let qdrant_url = env::var("QDRANT_URL")
        .unwrap_or_else(|_| "http://localhost:6333".to_string());
    
    let client = QdrantClient::from_url(&qdrant_url).build()?;
    
    // Create collection if it doesn't exist
    let collection_name = "infinitebase_nodes";
    let collections_list = client.list_collections().await?;
    
    if !collections_list.collections.iter().any(|c| c.name == collection_name) {
        client.create_collection(&qdrant_client::qdrant::CreateCollection {
            collection_name: collection_name.to_string(),
            vectors_config: Some(qdrant_client::qdrant::VectorsConfig {
                config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                    qdrant_client::qdrant::VectorParams {
                        size: 384, // Using a common embedding size
                        distance: qdrant_client::qdrant::Distance::Cosine.into(),
                        ..Default::default()
                    }
                )),
            }),
            ..Default::default()
        }).await?;
    }
    
    Ok(client)
}