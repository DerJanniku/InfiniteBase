# Security Model

## Security Principles
- Local-first default.
- Explicit consent before sending data to external AI providers.
- Least privilege for agent integrations.

## Threats & Controls
- Data exfiltration via AI API:
  - control: consent gate + clear provider warning
  - control: per-provider allowlist and scoped context
- Accidental destructive edits:
  - control: soft-delete (Visual Trash)
  - control: undo/redo + snapshots
- Agent abuse / runaway loops:
  - control: action rate limit
  - control: daily budget ceiling
  - control: timeout + max action batch size

## Data Protection
- Keep all board data local by default.
- Sensitive node zones should support encryption/lock policy (phase 2).
- Store audit logs for all automated modifications.

## Access Model (MVP)
- API is localhost-focused for development.
- Agent endpoints should require token auth before public exposure.
- Never expose host filesystem root to agent runtime.

## Operational Guidance
- Rotate API keys regularly.
- Keep Docker images updated.
- Do not commit secrets to git.
- Use `.env` for local credentials.
