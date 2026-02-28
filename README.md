# InfiniteBase

[![CI](https://github.com/DerJanniku/InfiniteBase/actions/workflows/ci.yml/badge.svg)](https://github.com/DerJanniku/InfiniteBase/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-MVP%20Development-orange)](docs/ROADMAP.md)

InfiniteBase is a local-first infinite canvas workspace and visual file explorer.
It combines spatial organization, file-native nodes, and API-first agent automation for personal knowledge work.

## Keywords
Local-first, infinite canvas, visual file explorer, spatial knowledge management, Rust backend, Next.js, tldraw, self-hosted, BYOA (Bring Your Own Agent), privacy-first.

## Problem Statement
Traditional folder-based workflows are rigid and context-poor. Information is spread across files, notes, links, and tools, which increases friction when switching between school, work, and personal projects.

## Solution
InfiniteBase provides a single spatial workspace where files and notes are represented as nodes on an infinite canvas. External agents can read context and perform controlled actions via API, while data remains local by default.

## Current Scope
- Infinite canvas frontend
- Node CRUD API
- File upload endpoint and file node creation
- Soft delete behavior (Visual Trash model)
- Agent context/action API stubs

## Architecture
- Frontend: Next.js + React + tldraw
- Backend: Rust + Axum + SQLx
- Database: PostgreSQL
- Vector store: Qdrant
- Runtime: Docker Compose

See details:
- [Architecture](docs/ARCHITECTURE.md)
- [API](docs/API.md)
- [Security](docs/SECURITY.md)
- [Roadmap](docs/ROADMAP.md)
- [AI Build Plan](docs/AI_BUILD_PLAN.md)

## Quick Start
Prerequisites:
- Docker + Docker Compose
- Node.js 20+ (optional for frontend-only local dev)

Run full stack:
```bash
docker compose up --build
```

Service endpoints:
- Frontend: `http://localhost:3000`
- Backend: `http://localhost:8080`
- Health: `http://localhost:8080/health`
- Qdrant: `http://localhost:6333`

Frontend-only development:
```bash
cd frontend
npm ci
npm run dev
```

## API Overview
Base URL: `http://localhost:8080`

- `GET /health`
- `GET /api/v1/nodes`
- `POST /api/v1/nodes`
- `GET /api/v1/nodes/:id`
- `PUT /api/v1/nodes/:id`
- `PATCH /api/v1/nodes/:id`
- `DELETE /api/v1/nodes/:id`
- `POST /api/v1/files/upload`
- `GET /api/v1/canvas/context`
- `POST /api/v1/agent/action`

Contract and payloads: [docs/API.md](docs/API.md)

## Repository Structure
- `frontend/` Next.js application
- `backend/` Rust API service
- `docs/` technical documentation
- `.github/` CI and contribution templates

## Project Status
The repository is in active MVP development. Core architecture is in place, but major functionality, reliability, and production hardening tasks remain open.

Track progress in Issues: <https://github.com/DerJanniku/InfiniteBase/issues>

## Contribution
- Read [Contributing Guide](docs/CONTRIBUTING.md)
- Open feature/bug issues with clear reproduction steps
- Keep pull requests scoped and documented

## Maintainer
- GitHub: [@DerJanniku](https://github.com/DerJanniku)
- Contact: `jannik.maier.jm@proton.me`

## License
MIT. See [LICENSE](LICENSE).
