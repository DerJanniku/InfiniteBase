# Contributing

## Scope
Contributions should align with local-first architecture and the AI Build Plan.

## Dev Setup
1. Install Docker + Docker Compose.
2. Run `docker compose up --build`.
3. Validate services:
- frontend: `http://localhost:3000`
- backend: `http://localhost:8080`

## Contribution Rules
- Keep changes focused and atomic.
- Add docs for any API or behavior change.
- Avoid introducing cloud dependencies without discussion.
- Preserve backward compatibility where feasible.

## Commit Guidance
- Use clear messages:
  - `feat(api): add node soft-delete endpoint`
  - `fix(frontend): prevent heavy preview render offscreen`
  - `docs(architecture): clarify agent action flow`

## Pull Request Checklist
- [ ] Code builds locally.
- [ ] API contracts documented (`docs/API.md`).
- [ ] Security impact considered (`docs/SECURITY.md`).
- [ ] Roadmap alignment checked (`docs/ROADMAP.md`).
- [ ] No secrets committed.

## Reporting Bugs
Please include:
- expected behavior
- actual behavior
- steps to reproduce
- logs/screenshots if relevant
