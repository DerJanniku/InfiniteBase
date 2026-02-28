mod api;
mod database;
mod models;
mod services;

use api::AppState;
use axum::{
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use qdrant_client::qdrant_client::QdrantClient;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = database::connect(&db_url).await;

    // Connect to Qdrant
    let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".into());
    let qdrant_client = QdrantClient::from_url(&qdrant_url).build()?;
    
    let state = AppState {
        db_pool: pool.clone(),
        qdrant_client: Arc::new(qdrant_client),
    };

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/nodes", get(api::node_handler::list_nodes))
        .route("/api/v1/nodes", post(api::node_handler::create_node))
        .route("/api/v1/nodes/:id", get(api::node_handler::get_node))
        .route("/api/v1/nodes/:id", put(api::node_handler::update_node))
        .route("/api/v1/nodes/:id", patch(api::node_handler::patch_node))
        .route("/api/v1/nodes/:id", delete(api::node_handler::delete_node))
        .route("/api/v1/upload", post(api::node_handler::upload_file))
        .route("/api/v1/files/upload", post(api::node_handler::upload_file))
        .route("/api/v1/canvas/context", get(api::node_handler::get_canvas_context))
        .route("/api/v1/agent/action", post(api::node_handler::agent_action))
        .nest_service("/api/v1/previews", ServeDir::new("/app/uploads/previews"))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("InfiniteBase backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}
