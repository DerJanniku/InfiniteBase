use sqlx::PgPool;
use qdrant_client::qdrant::QdrantClient;
use std::sync::Arc;

pub mod node_handler;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub qdrant_client: Arc<QdrantClient>,
}