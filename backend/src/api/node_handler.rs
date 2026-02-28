use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use sqlx::{Postgres, query, query_as};
use crate::api::AppState;
use crate::models::node::{Node, CreateNodeRequest, PatchNodeRequest};

pub async fn list_nodes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Node>>, StatusCode> {
    let nodes = query_as::<Postgres, Node>("SELECT * FROM nodes WHERE deleted = FALSE ORDER BY created_at DESC")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(nodes))
}

pub async fn get_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Node>, StatusCode> {
    let node = query_as::<Postgres, Node>("SELECT * FROM nodes WHERE id = $1 AND deleted = FALSE")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    node.map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_node(
    State(state): State<AppState>,
    Json(payload): Json<CreateNodeRequest>,
) -> Result<(StatusCode, Json<Node>), StatusCode> {
    let node = query_as::<Postgres, Node>(
        "INSERT INTO nodes (node_type, position_x, position_y, z_index, content, metadata, connections)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(&payload.node_type)
    .bind(payload.position_x)
    .bind(payload.position_y)
    .bind(payload.z_index)
    .bind(sqlx::types::Json(&payload.content))
    .bind(sqlx::types::Json(&payload.metadata))
    .bind(sqlx::types::Json(Vec::<crate::models::node::Connection>::new()))
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Create node error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(node)))
}

pub async fn update_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateNodeRequest>,
) -> Result<Json<Node>, StatusCode> {
    let node = query_as::<Postgres, Node>(
        "UPDATE nodes
         SET node_type = $1, position_x = $2, position_y = $3, z_index = $4, content = $5, metadata = $6, updated_at = NOW()
         WHERE id = $7 AND deleted = FALSE
         RETURNING *",
    )
    .bind(&payload.node_type)
    .bind(payload.position_x)
    .bind(payload.position_y)
    .bind(payload.z_index)
    .bind(sqlx::types::Json(&payload.content))
    .bind(sqlx::types::Json(&payload.metadata))
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(node))
}

pub async fn patch_node(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_payload): Json<PatchNodeRequest>,
) -> Result<Json<Node>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    query::<Postgres>("UPDATE nodes SET deleted = TRUE, updated_at = NOW() WHERE id = $1 AND deleted = FALSE")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn upload_file(
    State(_state): State<AppState>,
    _multipart: Multipart,
) -> Result<Json<Node>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_canvas_context(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let nodes = query_as::<Postgres, Node>("SELECT * FROM nodes WHERE deleted = FALSE")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "nodes": nodes,
        "total_count": nodes.len()
    })))
}

pub async fn agent_action(
    State(_state): State<AppState>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({"status": "ok"})))
}
