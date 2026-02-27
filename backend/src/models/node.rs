use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Node {
    pub id: Uuid,
    pub node_type: String,
    pub position_x: f64,
    pub position_y: f64,
    pub z_index: i32,
    pub content: Json<NodeContent>,
    pub metadata: Json<NodeMetadata>,
    pub connections: Json<Vec<Connection>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeContent {
    pub file_path: Option<String>,
    pub preview_url: Option<String>,
    pub text_content: Option<String>,
    pub custom_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMetadata {
    pub tags: Vec<String>,
    pub locked: bool,
    pub last_modified_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub to_node: Uuid,
    pub connection_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNodeRequest {
    pub node_type: String,
    pub position_x: f64,
    pub position_y: f64,
    pub z_index: i32,
    pub content: NodeContent,
    pub metadata: NodeMetadata,
}

#[derive(Debug, Deserialize)]
pub struct PatchNodeRequest {
    pub node_type: Option<String>,
    pub position_x: Option<f64>,
    pub position_y: Option<f64>,
    pub z_index: Option<i32>,
    pub content: Option<NodeContent>,
    pub metadata: Option<NodeMetadata>,
}
