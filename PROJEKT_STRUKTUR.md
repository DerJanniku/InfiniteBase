# 🚀 InfiniteBase - Projekt Roadmap & Entwicklungsplan

> **Status**: Konzeptphase / MVP-Entwicklung (2026)  
> **Vision**: "Lokales visuelles Gehirn"  
> **Entwickler**: Jannik Maier (12. Klasse FOS)

---

## 📋 Projekt-Übersicht

InfiniteBase ist ein Open-Source "Spatial OS" und Visual File Explorer, der klassische Ordnerstrukturen durch ein unendliches Canvas ersetzt. Local-first mit Docker, offene API für KI-Agenten.

---

## 🏗️ Tech-Stack

| Komponente | Technologie | Status |
|------------|-------------|--------|
| Backend | Rust + Axum | ✅ Existiert |
| Datenbank | PostgreSQL + Qdrant (RAG) | ✅ Docker bereit |
| Frontend | React/Next.js + tldraw | ❌ Fehlt |
| Infrastruktur | Docker Compose | ✅ Existiert |

---

## 📦 Aktueller Stand (Bestandsaufnahme)

### ✅ Bereits vorhanden

- [x] `docker-compose.yml` - Vollständiges Setup mit PostgreSQL, Qdrant, Rust-Backend, Next.js
- [x] `backend/Cargo.toml` - Rust Dependencies (axum, sqlx, qdrant, etc.)
- [x] `backend/src/main.rs` - Axum Server mit CORS, Migrationen
- [x] `backend/src/models/node.rs` - Node-Datenstruktur (JSON-Schema!)
- [x] `backend/src/api/node_handler.rs` - CRUD API Endpunkte
- [x] `backend/src/database/mod.rs` - PostgreSQL Connection Pool
- [x] `backend/migrations/` - SQL Migrationen
- [x] `AI_BUILD_PLAN.md` - Master-Prompt für KI-Agenten

### ❌ Noch zu erstellen

- [ ] Frontend (Next.js + tldraw)
- [ ] Dockerfile für Frontend
- [ ] Integration Frontend ↔ Backend
- [ ] Drag & Drop Funktionalität
- [ ] AI Agent API Endpoints

---

## 🎯 Entwicklungs-Phasen

### Phase 1: Fundament ✅ (Fast fertig)

- [x] Docker Compose Setup
- [x] Rust Backend mit CRUD
- [x] PostgreSQL + Qdrant Integration
- [ ] **TODO**: CORS Dependency in Cargo.toml fixen

### Phase 2: Frontend & Canvas (Aktuell)

- [ ] Next.js Projekt erstellen
- [ ] tldraw Canvas Engine integrieren
- [ ] Dark Mode + Punktraster Design
- [ ] Frontend ↔ Backend API Verbindung

### Phase 3: Drag & Drop

- [ ] Dateien auf Canvas ziehen
- [ ] File-Upload Handler in Rust
- [ ] Vorschau-Generierung (PDF, Bilder)

### Phase 4: AI Agent Integration

- [ ] `/api/v1/canvas/context` Endpoint
- [ ] Prompt-Node Implementierung
- [ ] RAG Pipeline (Text-Extraktion → Qdrant)

---

## 🔧 Technische Details

### Node JSON Schema (WICHTIG!)

```json
{
  "id": "node_8f72a1b9",
  "type": "file_pdf",
  "position": { "x": 1450.5, "y": -320.0, "z_index": 5 },
  "content": {
    "file_path": "/local/data/fos/mathe.pdf",
    "preview_url": "/api/preview/node_8f72a1b9.png",
    "text_content": "Extrahierter Text für RAG..."
  },
  "metadata": {
    "tags": ["#mathe", "#prüfung"],
    "locked": false,
    "last_modified_by": "agent_gemini_01"
  },
  "connections": [{ "to_node": "node_9a8b7c6d", "type": "prompt_link" }]
}
```

### API Endpoints

| Methode | Route | Beschreibung |
|---------|-------|--------------|
| GET | `/api/v1/nodes` | Alle Nodes laden |
| POST | `/api/v1/nodes` | Node erstellen |
| PUT | `/api/v1/nodes/:id` | Node aktualisieren |
| DELETE | `/api/v1/nodes/:id` | Soft-Delete (Visual Trash) |
| POST | `/api/v1/files/upload` | Datei-Upload |
| GET | `/api/v1/canvas/context` | RAG Context für KI-Agenten |

---

## 🎨 UI/UX Spezifikationen

### Design

- **Theme**: Radikaler Dark Mode
- **Background**: OLED-Schwarz `#000000`
- **Grid**: Punktraster (Dots) `#222222`
- **Keine harten Linien!**

### Features

- Lazy Loading (nur sichtbare Nodes laden)
- Magnetisches Snapping
- Mini-Map Navigation
- Fokus-Modus

---

## 🧪 Testing Checklist

- [ ] Backend startet ohne Fehler
- [ ] Datenbank-Migrationen laufen
- [ ] API gibt JSON zurück
- [ ] Frontend verbindet sich mit Backend
- [ ] Canvas rendert mit tldraw

---

## 📝 Nächste Schritte

1. **Sofort**: Cargo.toml um `tower-http` ergänzen
2. **Dann**: Next.js Frontend erstellen
3. **Dann**: tldraw Canvas integrieren
4. **Dann**: API Verbindung testen
5. **Dann**: Drag & Drop implementieren

---

## 🏆 Erfolgs-Kriterien

- MVP: Funktionierender Canvas mit CRUD
- FOS-Nutzung: PDF/Notizen auf Canvas organisieren
- AI: Gemini API Integration für Context-Verständnis

---

*Zuletzt aktualisiert: 28.02.2026*

