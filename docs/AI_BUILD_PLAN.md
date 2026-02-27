# InfiniteBase - AI Master Build Plan

Dieser Plan ist der verbindliche Bauplan fuer eine Coding-AI, die InfiniteBase implementiert.

## 1. Produktziel
InfiniteBase ist ein local-first, self-hosted Visual File Explorer auf einem Infinite Canvas. Es ersetzt starre Ordnerstrukturen durch eine visuelle Arbeitsflaeche mit Nodes.

## 2. Kernprinzipien
- Local-first: Daten bleiben lokal.
- Open Source Core.
- Bring Your Own Agent (BYOA): Keine fest verbaute AI, nur offene API-Anbindung.
- Privacy by default.

## 3. Zielnutzer
- Primaer: Schueler, Entwickler, Power-User mit Fokus auf Uebersicht und Datenschutz.
- Sekundaer: Freelancer, Wissensarbeiter, Kreative.

## 4. Tech-Stack (verbindlich)
- Frontend: Next.js (React), Tailwind CSS, tldraw.
- Backend: Rust (Axum bevorzugt).
- Daten: PostgreSQL + pgvector (oder Qdrant).
- Orchestrierung: Docker Compose.
- Desktop: Electron (autostartfaehig).

Keine Abweichungen ohne explizite Rueckfrage.

## 5. UX/UI Vorgaben
- Dark-First Design.
- Hintergrund: `#000000`.
- Dot-Grid: `#222222` (Punktraster, keine Linien).
- Snapping fuer Nodes.
- Focus Mode (blendert UI aus).
- Mini-Map fuer Navigation.
- Lazy Loading / Viewport Culling fuer Performance.

## 6. Datenmodell (Node-Schema)
Jedes Element auf dem Canvas ist ein Node.

```json
{
  "id": "node_8f72a1b9",
  "type": "file_pdf",
  "position": {
    "x": 1450.5,
    "y": -320.0,
    "z_index": 5
  },
  "content": {
    "file_path": "/local/data/fos/mathe_mitternachtsformel.pdf",
    "preview_url": "/api/preview/node_8f72a1b9.png",
    "text_content": "Extrahierter Text fuer RAG"
  },
  "metadata": {
    "tags": ["#mathe", "#pruefung"],
    "locked": false,
    "created_at": "2026-02-28T12:00:00Z",
    "last_modified_by": "agent_gemini_01"
  },
  "connections": [
    {
      "to_node": "node_9a8b7c6d",
      "type": "prompt_link"
    }
  ]
}
```

## 7. API-Anforderungen
- `GET /api/v1/nodes`
- `POST /api/v1/nodes`
- `PATCH /api/v1/nodes/:id`
- `DELETE /api/v1/nodes/:id` (Soft Delete -> Visual Trash)
- `GET /api/v1/canvas/context` (agent-read)
- `POST /api/v1/canvas/actions` (agent-write)
- `POST /api/v1/files/upload` (Drag-and-Drop)

## 8. AI-Agent Regeln
- Agenten duerfen nur ueber API arbeiten.
- Kein direkter Root-Dateisystemzugriff.
- Prompt-Node + verbundene Nodes definieren den Arbeitskontext.
- Kosten-Airbag: Request-Limits, Tagesbudget, Timeouts.
- Audit-Log: Jede Agent-Aktion mit Zeitstempel.

## 9. Dateisystem-Synchronisation
- Zwei-Wege-Sync zwischen Canvas und lokaler Dateiablage.
- Datei-Tracking ueber stabile IDs + Pfad-Mapping.
- Konflikterkennung bei externen Aenderungen.
- Rename/Move robust handhaben.

## 10. Sicherheit & Datenintegritaet
- Visual Trash statt Hard Delete.
- Undo/Redo und History.
- Snapshots (z. B. Lernstand Mai).
- Optional verschluesselte Bereiche fuer sensitive Nodes.
- Warnhinweis vor externen Cloud-AI Calls.

## 11. Performance-Ziele
- 10.000 Nodes ohne UI-Kollaps.
- PDF/Video lazy laden.
- Thumbnails im Zoom-Out.
- Heavy Rendering nur im Sichtbereich.

## 12. MVP Roadmap (strict sequence)
1. Infrastruktur
- Docker Compose mit `frontend`, `backend`, `db`.
- Rust Backend Grundgeruest + Healthcheck.

2. Canvas Basis
- Next.js + tldraw + Dot-Grid Theme.
- Nodes laden/speichern via API.

3. File Nodes
- Drag-and-Drop Upload.
- Previews (Bild/PDF-First-Page).

4. Agent API
- Context Endpoint + Action Endpoint.
- Prompt-Node Workflow.

5. Stabilitaet
- Visual Trash, History, Snapshots.
- Konflikterkennung und Recovery.

## 13. Repository-Dokumentation (Pflicht)
- `README.md`: Vision, Features, Quickstart.
- `docs/ARCHITECTURE.md`: Komponenten, Datenfluesse.
- `docs/API.md`: Endpunkte, Payloads, Fehlercodes.
- `docs/SECURITY.md`: Threat Model, Policies.
- `docs/ROADMAP.md`: MVP -> Phase 2/3.
- `docs/CONTRIBUTING.md`: Dev Setup, Standards.

## 14. Definition of Done (MVP)
- Infinite Canvas laeuft lokal im Browser/Electron.
- Dateien per Drag-and-Drop als Nodes sichtbar.
- Nodes werden persistent gespeichert.
- Agent kann Context lesen und neue Nodes erstellen.
- Keine Daten gehen ohne Zustimmung in externe Cloud.

## 15. Build-Prompt fuer jede Coding-AI
Verwende diesen Plan als einzige Quelle der Wahrheit. Implementiere strikt nach MVP-Reihenfolge, liefere pro Schritt:
1. Code
2. Tests
3. Migrations/Setup
4. Doku-Update in `docs/`
Stoppe nach jedem Schritt und fordere Review an.
