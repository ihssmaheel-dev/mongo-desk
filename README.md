# MongoDesk

Ultra-fast, lightweight, modern MongoDB desktop client for Windows.

## Features

- **Connection Manager** — Add, edit, delete, test connections with groups and favorites
- **Database/Collection Explorer** — Lazy-loaded tree with search and context menus
- **Document Viewer** — Table, JSON, and tree views with virtualization
- **Query Runner** — CodeMirror editor with MongoDB syntax highlighting and autocomplete
- **Aggregation Pipeline Builder** — Visual drag-and-drop builder with 12 core stages
- **Index Manager** — View, create, and delete indexes
- **Schema Explorer** — Auto-detect schema with field analysis
- **Export** — Streaming JSON and CSV export
- **Backup & Restore** — Streaming backup with progress reporting
- **Settings** — Dark/light theme, keyboard shortcuts, editor configuration

## Tech Stack

- **Frontend:** Svelte 5, TypeScript, Tailwind CSS, CodeMirror 6, TanStack Virtual
- **Backend:** Rust, Tauri v2, MongoDB Driver v3
- **Database:** SQLite (local storage)
- **Platform:** Windows-only (v1.0)

## Hardware Requirements

- 4-core CPU
- 8 GB RAM
- SSD

## Development

### Prerequisites

- Node.js 20+
- Rust 1.75+
- MongoDB 8.x (or 6.0+)

### Setup

```bash
# Install dependencies
npm install

# Start development
npm run dev
```

### Build

```bash
# Build for production
npm run build

# Build Tauri app
cargo tauri build
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Installer Size | < 35 MB |
| Startup Time | < 1 second |
| Idle RAM | 60–120 MB |
| Active RAM | < 250 MB |
| Collection Open | < 200 ms |
| Scroll FPS | 60 FPS |
| Export 10 GB | Constant memory |
| Backup 10 GB | Constant memory |
| Open 1M Docs | Smooth |

## License

MIT
