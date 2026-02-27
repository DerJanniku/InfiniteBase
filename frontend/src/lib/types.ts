// InfiniteBase API Types
// Must match the Rust backend models!

export interface NodePosition {
  x: number;
  y: number;
  z_index: number;
}

export interface NodeContent {
  file_path?: string;
  preview_url?: string;
  text_content?: string;
  custom_data?: Record<string, unknown>;
}

export interface NodeMetadata {
  tags: string[];
  locked: boolean;
  last_modified_by?: string;
}

export interface Connection {
  to_node: string;
  connection_type: string;
}

export interface CanvasNode {
  id: string;
  node_type: string;
  position_x: number;
  position_y: number;
  z_index: number;
  content: NodeContent;
  metadata: NodeMetadata;
  connections: Connection[];
  created_at: string;
  updated_at: string;
}

export interface CreateNodeRequest {
  node_type: string;
  position_x: number;
  position_y: number;
  z_index: number;
  content: NodeContent;
  metadata: NodeMetadata;
}

export interface UpdateNodeRequest extends CreateNodeRequest {}

export interface CanvasContext {
  nodes: CanvasNode[];
  tags: string[];
  connections: Connection[];
}

// API Response types
export interface ApiResponse<T> {
  data: T;
  error?: string;
}

export interface UploadResponse {
  id: string;
  file_path: string;
  preview_url: string;
}

// Node Types
export const NODE_TYPES = {
  FILE_PDF: "file_pdf",
  FILE_IMAGE: "file_image",
  FILE_VIDEO: "file_video",
  FILE_DOC: "file_doc",
  TEXT_NOTE: "text_note",
  PROMPT_NODE: "prompt_node",
  LINK: "link",
  GROUP: "group",
} as const;

export type NodeType = typeof NODE_TYPES[keyof typeof NODE_TYPES];

