use std::sync::Arc;
use qdrant_client::qdrant_client::QdrantClient;
use sqlx::PgPool;

pub mod node_handler;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub qdrant_client: Arc<QdrantClient>,
}
