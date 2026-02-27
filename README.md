# 🧠 InfiniteBase - The Personal Canvas OS

> **"Dein lokales visuelles Gehirn für Schule, Arbeit und Leben."**

InfiniteBase ist ein Open-Source **Spatial OS** und ein **Visual File Explorer**, der die klassische, starre Ordnerstruktur durch ein unendliches, interaktives Canvas ersetzt. Die App ist **Local-first** konzipiert, läuft via Docker auf dem Localhost und ermöglicht die nahtlose Einbindung von KI-Agenten, die den Kontext auf dem Board lesen und aktiv mitgestalten können.

![InfiniteBase Branding](https://raw.githubusercontent.com/DerJannik/.github/main/branding/infinitebase_banner.png)

## 🚀 Vision & Key Features

InfiniteBase bricht mit traditionellen Paradigmen. Statt Dateien in tief verschachtelten Ordnern zu verstecken, platzierst du sie dort, wo sie für deinen Workflow Sinn ergeben.

- **Visual File Explorer:** PDFs, Videos, Bilder und Dokumente sind direkt auf dem Canvas abspielbar und scrollbar. 
- **AI Agent API (BYOA):** "Bring Your Own Agent". Nutze Gemini, Antigravity oder lokale LLMs, die via API auf deinen Canvas-Kontext zugreifen, Nodes erstellen oder Inhalte zusammenfassen.
- **Prompt-Nodes:** Steuere deine KI-Agenten visuell. Ziehe Linien zwischen einem Befehl (Prompt-Node) und deinen Dateien, um Aktionen auszulösen.
- **FOS-Power (Schul-Modus):** Speziell für Schüler optimiert. iPad-Handschrift wird via OCR erkannt, Lernkarten werden automatisch generiert und Math-Formeln (Mitternachtsformel!) triggern interaktive Plots.
- **OLED Black Design:** Ein minimalistisches, tiefschwarzes Interface mit einem dezenten Punktraster – schont die Augen und den Akku.

## 🛠 Tech-Stack

InfiniteBase setzt auf maximale Performance und Datensouveränität:

| Komponente | Technologie | Zweck |
| :--- | :--- | :--- |
| **Backend** | [Rust](https://www.rust-lang.org/) (Axum) | High-Performance Datei-Operationen & API |
| **Frontend** | [Next.js](https://nextjs.org/) + [tldraw](https://tldraw.dev/) | Infinite Canvas Engine |
| **Datenbank** | [PostgreSQL](https://www.postgresql.org/) + [pgvector](https://github.com/pgvector/pgvector) | Relationale Daten & Vektor-Suche (RAG) |
| **Vector DB** | [Qdrant](https://qdrant.tech/) | Skalierbares KI-Gedächtnis |
| **Orchestrierung** | [Docker Compose](https://docs.docker.com/compose/) | One-Click Localhost Deployment |

## 📦 Installation & Start

Stelle sicher, dass **Docker** und **Docker Compose** installiert sind.

1. **Repo klonen:**
   ```bash
   git clone https://github.com/DerJannik/InfiniteBase.git
   cd InfiniteBase
   ```

2. **Starten:**
   ```bash
   docker-compose up --build
   ```

3. **Zugriff:**
   Öffne [http://localhost:3000](http://localhost:3000) in deinem Browser (oder nutze den Electron-Wrapper).

## 🤖 AI Agent Integration

InfiniteBase bietet eine offene REST-API unter `/api/v1/canvas/context`. Externe Skripte (z.B. in Python) können das Board-JSON lesen, analysieren und neue Nodes zurückschreiben.

```json
{
  "action": "create_node",
  "type": "note",
  "content": "Zusammenfassung der Mathestunde...",
  "position": { "x": 500, "y": 200 }
}
```

## 📜 Lizenz & Community

Dieses Projekt lizenziert unter der **MIT-Lizenz**. InfiniteBase ist von der Community für die Community. 

- **Mitwirken:** Forke das Repo und erstelle einen Pull Request.
- **Diskussion:** Nutze die GitHub Discussions für Feature-Wünsche oder Fragen.
- **Community-Name:** Wir nennen uns **Basers**.

---

*Made with ❤️ by [DerJannik](https://github.com/DerJannik) | Local-First. Privacy-Focused.*
