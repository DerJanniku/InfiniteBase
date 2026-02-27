use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::path::Path as StdPath;
use tokio::fs;
use uuid::Uuid;

use crate::models::node::{
    Connection, CreateNodeRequest, Node, NodeContent, NodeMetadata, PatchNodeRequest,
};

#[derive(Serialize)]
struct ApiErrorBody {
    error: ApiErrorInfo,
}

#[derive(Serialize)]
struct ApiErrorInfo {
    code: &'static str,
    message: String,
}

fn api_error(status: StatusCode, code: &'static str, message: impl Into<String>) -> (StatusCode, Json<ApiErrorBody>) {
    (
        status,
        Json(ApiErrorBody {
            error: ApiErrorInfo {
                code,
                message: message.into(),
            },
        }),
    )
}

pub async fn list_nodes(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Node>>, (StatusCode, Json<ApiErrorBody>)> {
    let nodes = sqlx::query_as::<_, Node>(
        "SELECT * FROM nodes WHERE deleted = FALSE ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Error listing nodes: {:?}", e);
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to list nodes",
        )
    })?;

    Ok(Json(nodes))
}

pub async fn get_node(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Node>, (StatusCode, Json<ApiErrorBody>)> {
    let node = sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE id = $1 AND deleted = FALSE")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error loading node {}: {:?}", id, e);
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Failed to load node",
            )
        })?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "NOT_FOUND", "Node not found"))?;

    Ok(Json(node))
}

pub async fn create_node(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateNodeRequest>,
) -> Result<(StatusCode, Json<Node>), (StatusCode, Json<ApiErrorBody>)> {
    let node = sqlx::query_as::<_, Node>(
        "INSERT INTO nodes (node_type, position_x, position_y, z_index, content, metadata, connections)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(payload.node_type)
    .bind(payload.position_x)
    .bind(payload.position_y)
    .bind(payload.z_index)
    .bind(serde_json::to_value(payload.content).map_err(|_| {
        api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Invalid content payload")
    })?)
    .bind(serde_json::to_value(payload.metadata).map_err(|_| {
        api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Invalid metadata payload")
    })?)
    .bind(serde_json::to_value(Vec::<Connection>::new()).map_err(|_| {
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to encode connections",
        )
    })?)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Error creating node: {:?}", e);
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to create node",
        )
    })?;

    Ok((StatusCode::CREATED, Json(node)))
}

pub async fn update_node(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateNodeRequest>,
) -> Result<Json<Node>, (StatusCode, Json<ApiErrorBody>)> {
    let node = sqlx::query_as::<_, Node>(
        "UPDATE nodes
         SET node_type = $1, position_x = $2, position_y = $3, z_index = $4, content = $5, metadata = $6, updated_at = NOW()
         WHERE id = $7 AND deleted = FALSE
         RETURNING *",
    )
    .bind(payload.node_type)
    .bind(payload.position_x)
    .bind(payload.position_y)
    .bind(payload.z_index)
    .bind(serde_json::to_value(payload.content).map_err(|_| {
        api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Invalid content payload")
    })?)
    .bind(serde_json::to_value(payload.metadata).map_err(|_| {
        api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Invalid metadata payload")
    })?)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Error updating node {}: {:?}", id, e);
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to update node",
        )
    })?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "NOT_FOUND", "Node not found"))?;

    Ok(Json(node))
}

pub async fn patch_node(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<PatchNodeRequest>,
) -> Result<Json<Node>, (StatusCode, Json<ApiErrorBody>)> {
    let current = sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE id = $1 AND deleted = FALSE")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error loading node {} before patch: {:?}", id, e);
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Failed to load node",
            )
        })?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "NOT_FOUND", "Node not found"))?;

    let next_node_type = payload.node_type.unwrap_or(current.node_type);
    let next_position_x = payload.position_x.unwrap_or(current.position_x);
    let next_position_y = payload.position_y.unwrap_or(current.position_y);
    let next_z_index = payload.z_index.unwrap_or(current.z_index);
    let next_content = payload.content.unwrap_or(current.content.0);
    let next_metadata = payload.metadata.unwrap_or(current.metadata.0);

    let updated = sqlx::query_as::<_, Node>(
        "UPDATE nodes
         SET node_type = $1, position_x = $2, position_y = $3, z_index = $4, content = $5, metadata = $6, updated_at = NOW()
         WHERE id = $7 AND deleted = FALSE
         RETURNING *",
    )
    .bind(next_node_type)
    .bind(next_position_x)
    .bind(next_position_y)
    .bind(next_z_index)
    .bind(serde_json::to_value(next_content).map_err(|_| {
        api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Invalid content payload")
    })?)
    .bind(serde_json::to_value(next_metadata).map_err(|_| {
        api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Invalid metadata payload")
    })?)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Error patching node {}: {:?}", id, e);
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to patch node",
        )
    })?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "NOT_FOUND", "Node not found"))?;

    Ok(Json(updated))
}

pub async fn delete_node(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ApiErrorBody>)> {
    let result = sqlx::query("UPDATE nodes SET deleted = TRUE, updated_at = NOW() WHERE id = $1 AND deleted = FALSE")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error deleting node {}: {:?}", id, e);
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Failed to delete node",
            )
        })?;

    if result.rows_affected() == 0 {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            "Node not found",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn upload_file(
    State(pool): State<Pool<Postgres>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Node>), (StatusCode, Json<ApiErrorBody>)> {
    let mut file_path = None;
    let mut file_name = None;
    let mut position_x = 0.0;
    let mut position_y = 0.0;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Error reading multipart field: {:?}", e);
        api_error(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            format!("Invalid multipart payload: {e}"),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let file_name_str = field.file_name().unwrap_or("unknown").to_string();
            let data = field.bytes().await.map_err(|e| {
                tracing::error!("Error reading file data: {:?}", e);
                api_error(
                    StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    format!("Failed to read file bytes: {e}"),
                )
            })?;

            let upload_dir = StdPath::new("/app/uploads");
            if !upload_dir.exists() {
                fs::create_dir_all(upload_dir).await.map_err(|e| {
                    tracing::error!("Error creating upload directory: {:?}", e);
                    api_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "INTERNAL_ERROR",
                        format!("Failed to create upload directory: {e}"),
                    )
                })?;
            }

            let saved_path = upload_dir.join(&file_name_str);
            fs::write(&saved_path, &data).await.map_err(|e| {
                tracing::error!("Error saving file: {:?}", e);
                api_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    format!("Failed to save file: {e}"),
                )
            })?;

            file_path = Some(saved_path.to_string_lossy().to_string());
            file_name = Some(file_name_str);
        } else if name == "position_x" {
            let value = field.text().await.map_err(|e| {
                api_error(
                    StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    format!("Failed to read position_x: {e}"),
                )
            })?;
            position_x = value.parse().unwrap_or(0.0);
        } else if name == "position_y" {
            let value = field.text().await.map_err(|e| {
                api_error(
                    StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    format!("Failed to read position_y: {e}"),
                )
            })?;
            position_y = value.parse().unwrap_or(0.0);
        }
    }

    let file_path =
        file_path.ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Missing file"))?;
    let file_name =
        file_name.ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", "Missing filename"))?;

    let node_type = if file_name.to_lowercase().ends_with(".pdf") {
        "file_pdf"
    } else if file_name.to_lowercase().ends_with(".jpg")
        || file_name.to_lowercase().ends_with(".jpeg")
        || file_name.to_lowercase().ends_with(".png")
        || file_name.to_lowercase().ends_with(".gif")
    {
        "file_image"
    } else if file_name.to_lowercase().ends_with(".mp4")
        || file_name.to_lowercase().ends_with(".mov")
        || file_name.to_lowercase().ends_with(".avi")
    {
        "file_video"
    } else {
        "file_document"
    };

    let content = NodeContent {
        file_path: Some(file_path),
        preview_url: Some(format!("/api/preview/{}", Uuid::new_v4())),
        text_content: None,
        custom_data: Some(json!({ "original_filename": file_name })),
    };

    let metadata = NodeMetadata {
        tags: vec!["#uploaded".to_string()],
        locked: false,
        last_modified_by: Some("user".to_string()),
    };

    let node = sqlx::query_as::<_, Node>(
        "INSERT INTO nodes (node_type, position_x, position_y, z_index, content, metadata, connections)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(node_type)
    .bind(position_x)
    .bind(position_y)
    .bind(0)
    .bind(serde_json::to_value(content).map_err(|_| {
        api_error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Failed to encode content")
    })?)
    .bind(serde_json::to_value(metadata).map_err(|_| {
        api_error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Failed to encode metadata")
    })?)
    .bind(serde_json::to_value(Vec::<Connection>::new()).map_err(|_| {
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to encode connections",
        )
    })?)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Error creating file node: {:?}", e);
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Failed to create file node",
        )
    })?;

    Ok((StatusCode::CREATED, Json(node)))
}

pub async fn get_canvas_context(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiErrorBody>)> {
    let nodes = sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE deleted = FALSE")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error getting canvas context: {:?}", e);
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Failed to build canvas context",
            )
        })?;

    Ok(Json(json!({
        "nodes": nodes,
        "total_count": nodes.len(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn agent_action(
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiErrorBody>)> {
    tracing::info!("AI Agent action received: {:?}", payload);

    Ok(Json(json!({
        "status": "success",
        "message": "Action received and queued for processing",
        "action_id": Uuid::new_v4().to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
