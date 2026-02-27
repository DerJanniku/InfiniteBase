# 🧠 InfiniteBase - AI Master Build Plan

**System-Prompt für KI-Assistenten:** Lese dieses dokument komplett durch, bevor du code für dieses projekt generierst. Dies ist der architektur-bauplan für "InfiniteBase", einen lokalen Visual File Explorer mit Infinite Canvas und AI-Agenten-API. Halte dich strikt an diese vorgaben!

## 1. Core Concept & Vision

### Was bauen wir?
Ein lokales "Spatial OS". Ein unendliches Canvas, das klassische ordner ersetzt.

### Wer nutzt es?
Jannik (12. Klasse FOS). Fokus auf lernen, privatsphäre und workflows.

### Das wichtigste Feature: "Bring Your Own Agent"
Das system hat keine eigene KI, allerdings eine offene API für lokale oder externe agenten (Gemini, Antigravity), die den context auf dem board lesen und modifizieren können.

## 2. Tech-Stack (Strict Rules)

Die KI darf keine anderen technologien ohne rückfrage verwenden:

- **Frontend:** React (Next.js) + Tailwind CSS + tldraw (für die Infinite Canvas Engine)
- **Backend:** Rust (Axum oder Tauri Core) für blitzschnelle datei-operationen und das RAG-system
- **Datenbank:** PostgreSQL + eine Vektor-Datenbank (z.B. Qdrant oder pgvector) für die RAG-suche
- **Infrastruktur:** Alles MUSS über docker-compose auf dem localhost laufen. Keine cloud-dienste

## 3. Die Datenstruktur (Das Herzstück)

Das frontend und backend kommunizieren über JSON. Alles auf dem board ist ein "Node". Hier ist das basis-schema, das die KI für die datenbank und das frontend verwenden muss:

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
    "text_content": "Extrahierter Text für die Vektor-DB..."
  },
  "metadata": {
    "tags": ["#mathe", "#prüfung"],
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

**Notiz an die KI:** Agenten lesen dieses JSON über die API, um den context (wo liegt was, womit ist es verbunden) zu verstehen!

## 4. Design & UI Vorgaben

- **Theme:** Radikaler Dark Mode
- **Background:** OLED-Schwarz (#000000)
- **Grid:** Punktraster (Dots) in Dunkelgrau (#222222). Keine harten linien!
- **Interaktion:** Magnetisches einrasten (Snapping) muss aktiviert sein
- **Lazy Loading:** Rendere nur Nodes, die im sichtbaren bereich des viewports liegen, das die app bei 10.000 files nicht crasht

## 5. Entwicklungs-Roadmap (MVP Step-by-Step)

Die KI soll diese schritte nacheinander abarbeiten und nicht alles auf einmal bauen!

### Phase 1: Das Fundament (Docker & Rust)
1. Setup docker-compose.yml mit einem Rust-Backend container und einer PostgreSQL datenbank
2. Baue einfache CRUD-API-routen in Rust, um Nodes (das JSON von oben) zu speichern und zu laden

### Phase 2: Das Canvas (React & tldraw)
1. Setup Next.js frontend container in der docker-compose
2. Integriere tldraw mit dem schwarzen punktraster-design
3. Verbinde das frontend mit der Rust-API (Lade Nodes auf das Canvas)

### Phase 3: Drag & Drop (Der File Explorer)
1. Implementiere drag & drop in React
2. Wenn eine datei auf das board gezogen wird -> sende file an Rust -> Rust speichert file lokal, generiert ID und schickt das JSON zurück ans frontend

### Phase 4: Die Agenten-API (BYOA)
1. Baue in Rust eine offene REST-API (z.B. /api/v1/canvas/context)
2. Erlaube externen python-scripten (Antigravity) dieses JSON abzurufen, zu bearbeiten und veränderungen (neue Nodes) wieder auf das board zu pushen
3. Implementiere den "Prompt-Node" im frontend, der linien-verbindungen als context an die API schickt

## 6. Wichtige Regeln für die KI

1. **Local First:** Niemals externe APIs (außer CDN für libraries) ohne erlaubnis einbauen
2. **Sicherheit:** Baue einen "Visual Trash" ein. Lösche niemals datein direkt von der festplatte beim ersten klick
3. **Osterei:** Wenn irgendwo im text oder bild "Mitternachtsformel" erkannt wird, füge einen kommentar in den code ein, wie wir später die konfetti-animation triggern!

## 7. Node-Typen (Müssen unterstützt werden)

1. **File Nodes:** PDF, Images, Videos, Text-Dateien
2. **Text Nodes:** Sticky Notes, Markdown-Editor
3. **Prompt Nodes:** Spezielle nodes für KI-befehle
4. **Connection Nodes:** Linien/Verbindungen zwischen anderen nodes
5. **Folder Nodes:** Visuelle gruppierung von anderen nodes

## 8. API-Endpoints (Müssen implementiert werden)

### Backend (Rust)
- `GET /api/v1/nodes` - Alle nodes abrufen
- `POST /api/v1/nodes` - Neuen node erstellen
- `PUT /api/v1/nodes/:id` - Node aktualisieren
- `DELETE /api/v1/nodes/:id` - Node in visual trash verschieben
- `POST /api/v1/upload` - Datei hochladen und node erstellen
- `GET /api/v1/canvas/context` - Kompletten canvas context für KI-agenten
- `POST /api/v1/agent/action` - KI-agenten können actions ausführen

### Frontend (Next.js)
- `GET /api/preview/:node_id` - Vorschau-bilder generieren
- `GET /api/search?q=...` - Globale suche über nodes

## 9. Docker Compose Struktur

```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: infinitebase
      POSTGRES_USER: infinitebase
      POSTGRES_PASSWORD: localhost_only
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  qdrant:
    image: qdrant/qdrant
    ports:
      - "6333:6333"
    volumes:
      - qdrant_data:/qdrant/storage

  rust-backend:
    build: ./backend
    ports:
      - "8080:8080"
    volumes:
      - ./data:/app/data
      - ./uploads:/app/uploads
    depends_on:
      - postgres
      - qdrant

  nextjs-frontend:
    build: ./frontend
    ports:
      - "3000:3000"
    depends_on:
      - rust-backend

volumes:
  postgres_data:
  qdrant_data:
```

## 10. Projekt-Struktur

```
infinitebase/
├── AI_BUILD_PLAN.md          # Dieses dokument
├── docker-compose.yml        # Docker compose konfiguration
├── README.md                 # Projekt-dokumentation
├── backend/                  # Rust backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   ├── models/
│   │   └── database/
│   └── Dockerfile
├── frontend/                 # Next.js frontend
│   ├── package.json
│   ├── next.config.js
│   ├── src/
│   │   ├── app/
│   │   ├── components/
│   │   └── lib/
│   └── Dockerfile
├── data/                     # Lokale daten (wird von docker gemountet)
├── uploads/                  Hochgeladene dateien
└── docs/                     # Weitere dokumentation
```

## 11. Spezielle Features für die FOS

1. **OCR Integration:** iPad handschrift wird automatisch als text erkannt
2. **Präsentations-Modus:** Nodes können als slideshow präsentiert werden
3. **Mathe-Formel Erkennung:** Besondere unterstützung für mathematische formeln
4. **Lern-Karten:** Automatische generierung von lern-karten aus notes

## 12. Erfolgs-Metriken

- **Technisch:** App startet lokal via `docker-compose up`
- **Funktional:** Dateien können per drag & drop auf canvas gezogen werden
- **KI-Integration:** Externe python-scripts können über API auf canvas zugreifen
- **Persönlich:** Hilft Jannik bei der FOS-prüfungsvorbereitung

---

**Letzte Warnung an die KI:** Dieses projekt ist LOCAL-FIRST. Keine telemetrie, keine analytics, keine cloud-dienste. Alles bleibt auf dem localhost!

**Ready für Phase 1?** Starte mit der docker-compose.yml und dem Rust-backend!