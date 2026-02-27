# API Contract (MVP)

Base URL: `http://localhost:8080`

## Health
### `GET /health`
- Purpose: liveness probe
- Response: `200 { "ok": true }`

## Nodes
### `GET /api/v1/nodes`
Query params:
- `board_id` (optional)
- `viewport` (optional bounding box)

Response:
```json
{
  "nodes": [],
  "edges": []
}
```

### `POST /api/v1/nodes`
Request:
```json
{
  "type": "file_pdf",
  "position": { "x": 100, "y": 200, "z_index": 1 },
  "content": { "file_path": "/app/uploads/doc.pdf" },
  "metadata": { "tags": ["#schule"], "locked": false }
}
```

### `PATCH /api/v1/nodes/:id`
- Partial update for position/content/metadata.

### `DELETE /api/v1/nodes/:id`
- Soft-delete only.
- Node moves to Visual Trash.

## Files
### `POST /api/v1/files/upload`
- Multipart upload.
- Creates backing file + corresponding node payload.

Response (example):
```json
{
  "node_id": "node_123",
  "file_path": "/app/uploads/node_123.pdf",
  "preview_url": "/api/preview/node_123.png"
}
```

## Agents
### `GET /api/v1/canvas/context`
- Returns context slice for BYOA agents.
- Supports scope by prompt-node id and connected graph.

### `POST /api/v1/canvas/actions`
Request:
```json
{
  "agent_id": "agent_gemini_01",
  "actions": [
    {
      "type": "create_node",
      "payload": {
        "type": "note",
        "position": { "x": 400, "y": 220, "z_index": 3 },
        "content": { "text": "Summary" }
      }
    }
  ]
}
```

## Errors
Standard shape:
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "details"
  }
}
```

## Non-Functional API Rules
- Validate all agent actions.
- Enforce rate limits / budget guards for AI-triggered paths.
- Log mutating requests to audit history.
