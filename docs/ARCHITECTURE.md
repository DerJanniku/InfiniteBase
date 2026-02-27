# Architecture

## High-Level Components
- `nextjs-frontend`:
  - renders infinite canvas
  - handles drag-and-drop and node interactions
  - calls backend APIs
- `rust-backend`:
  - source of truth for node CRUD
  - manages file metadata and sync events
  - exposes agent endpoints for context/action
- `postgres`:
  - persistent relational store for nodes, edges, metadata, logs
- `qdrant`:
  - vector index for semantic search and context retrieval

## Data Flow
1. User drops a file on canvas.
2. Frontend uploads file (`/api/v1/files/upload`).
3. Backend stores file metadata, creates node record in Postgres.
4. Backend extracts text (where possible), writes embedding to Qdrant.
5. Frontend receives node payload and renders it.

## Agent Flow (BYOA)
1. External agent requests context (`GET /api/v1/canvas/context`).
2. Backend returns graph-aware payload (prompt node + connected nodes + metadata).
3. Agent returns actions (`POST /api/v1/canvas/actions`).
4. Backend validates, persists, broadcasts update.

## Synchronization Model
- Two-way sync between canvas nodes and underlying files.
- File events (rename/move/update) are mapped back to node IDs.
- Conflict policy:
  - detect divergence (mtime/hash/version)
  - preserve both versions when needed
  - mark node with conflict status for user resolution

## Performance Strategy
- Viewport culling: only render visible nodes.
- Lazy previews for heavy assets (PDF/video).
- Thumbnail-first rendering at low zoom levels.
- Batched API updates for move/resize operations.

## Failure Recovery
- Soft-delete with Visual Trash.
- Operation log for undo/redo.
- Snapshot restore points.
- Idempotent API handlers where possible.
