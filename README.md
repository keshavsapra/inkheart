<div align="center">
  <img src="frontend/public/static/logo.svg" alt="Inkheart logo" width="160" height="160">
  <h1>Inkheart</h1>
  <p><strong>Lightweight, self-hosted PDF & EPUB library indexer and web reader.</strong></p>

  <p>
    <a href="https://github.com/keshavsapra/inkheart"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License"></a>
    <img src="https://img.shields.io/badge/Platforms-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20ARM64-blue?logo=docker" alt="Platform">
    <img src="https://img.shields.io/badge/Formats-PDF%20%7C%20EPUB-orange" alt="Supported Formats">
    <a href="https://github.com/keshavsapra/inkheart/actions"><img src="https://github.com/keshavsapra/inkheart/actions/workflows/docker.yml/badge.svg" alt="Build Status"></a>
  </p>
</div>

<br>

<p align="center">
  <img src="screenshots/example.png" alt="Inkheart Screenshot" width="85%">
</p>

---

## 📖 About Inkheart

> **Note**: This repository is an enhanced fork of the original **Inkheart** project created by **[Nystik](https://github.com/Nystik-gh)**. It extends the original PDF library by introducing native **EPUB support**, multi-architecture GitHub Actions CI/CD (`linux/amd64` & `linux/arm64`), and web reader improvements.

**Inkheart** is a minimalist, self-hosted digital library designed around one core philosophy: **your filesystem is the source of truth**. 

Instead of requiring an external database, complex schemas, or manual book metadata curation, Inkheart scans your media directory as an on-disk folder tree. It extracts high-resolution covers, compiles an instant full-text search index, and provides sleek, integrated web readers for both **PDF** and **EPUB** documents.

Inkheart is intentionally lightweight and focused. It is not an unwieldy comic/manga scraper or a full-blown OPDS manager — it does one thing exceptionally well: gives you a fast, beautiful way to read and search your documents on desktop, tablet, and mobile.

---

## ✨ Features

- 📁 **Filesystem-Driven Navigation** — Preserves your exact folder hierarchy without requiring a database.
- 📑 **Dual Native Web Readers**:
  - **PDF Reader**: Built on PDF.js with continuous scrolling, page spread modes, thumbnails, and outline navigation.
  - **EPUB Reader**: Built on epub.js with table-of-contents navigation, responsive page turns, keyboard shortcuts, and swipe gestures.
- 🔍 **Blazing Fast Full-Text Search** — Powered by [Tantivy](https://github.com/quickwit-oss/tantivy). Indexes content across PDF pages and EPUB chapters with outline/heading boost. Only re-indexes modified files (`mtime`).
- 🎨 **Theme Consistency** — Adaptive light and dark modes across the app and document reading canvas.
- 🔖 **Reading Progress Tracking** — Automatically remembers your current page/CFI location in `localStorage`.
- 📌 **Sidebar Pins & Collections** — Pin favorite folders to your sidebar or group documents into custom collections.
- 🔗 **Direct Page & Location Linking** — Link directly to any page or chapter (e.g. `/reader/:id?page=4`), with OpenGraph social preview cards.
- 📱 **Mobile & Tablet Optimized** — Responsive drawer sidebars, auto-collapsing toolbars, and touch navigation.
- 🌐 **Multi-Architecture Support** — Prebuilt Docker images for **`linux/amd64`** and **`linux/arm64`** (runs natively on Ubuntu, Debian, Fedora, Raspberry Pi, Apple Silicon, and NAS platforms).
- 🔒 **Optional Authentication** — Simple drop-in Firebase Auth gate with account whitelisting.

---

## 🚀 Quickstart with Docker Compose

Create a `docker-compose.yml` file:

```yaml
services:
  inkheart:
    image: ghcr.io/keshavsapra/inkheart:latest
    container_name: inkheart
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - /path/to/your/books:/media         # Your PDFs and EPUBs
      - /path/to/covers:/covers            # Extracted cover images
      - /path/to/thumbnails:/thumbnails    # OpenGraph preview thumbnails
      - /path/to/config:/config            # Search index, settings, and pins
    environment:
      - BIND_ADDR=0.0.0.0
      - BIND_PORT=8080
      - SCAN_INTERVAL=600                  # Filesystem re-scan interval in seconds
      - TEXT_INDEX_ENABLED=true            # Enable full-text search indexing
```

Ensure your storage directories exist and have proper write permissions (Inkheart runs as non-root UID 1000 inside the container):
```bash
mkdir -p /path/to/covers /path/to/thumbnails /path/to/config /path/to/your/books
sudo chown -R 1000:1000 /path/to/covers /path/to/thumbnails /path/to/config
```
*(Alternatively, you can set `user: "0:0"` in your `docker-compose.yml` to allow the container to run with host root permissions).*

Start the container:
```bash
docker compose up -d
```
Open **`http://localhost:8080`** in your browser.

> [!NOTE]
> On the first startup with a large collection, building the full-text search index may take a few minutes. Because `/config` is persisted, subsequent restarts will only scan for new or modified files.

---

## 💻 Cross-Platform Deployment

Inkheart images are published as multi-architecture containers (`linux/amd64` and `linux/arm64`). Docker will automatically detect and pull the optimal binary for your host without any extra configuration:

- **Linux / VPS**: Ubuntu, Debian, Fedora, Arch, Alpine, etc.
- **Home Servers & NAS**: Synology, Unraid, TrueNAS SCALE, QNAP, or Portainer.
- **Single-Board Computers**: Raspberry Pi (4B / 5), Rock Pi, Orange Pi, and other ARM64 systems.
- **macOS / Windows**: Via Docker Desktop (native ARM64 execution on Apple Silicon M-series chips).

### Docker CLI Example
```bash
docker run -d \
  --name inkheart \
  --restart unless-stopped \
  -p 8080:8080 \
  -v /path/to/your/books:/media \
  -v /path/to/covers:/covers \
  -v /path/to/thumbnails:/thumbnails \
  -v /path/to/config:/config \
  ghcr.io/keshavsapra/inkheart:latest
```

---

## ⚙️ Configuration Reference

Inkheart supports configuration through environment variables or an `inkheart.toml` file.

| Environment Variable | Description | Default |
|---|---|---|
| `MEDIA_DIR` | Directory containing your PDF and EPUB files | `/media` |
| `IMAGE_DIR` | Directory where book covers are cached | `/covers` |
| `THUMBNAIL_DIR` | Directory for on-demand page preview thumbnails | `/thumbnails` |
| `CONFIG_DIR` | Directory storing user data, pins, collections, and search index | Parent dir of `CONFIG_PATH` |
| `CONFIG_PATH` | Optional path to an `inkheart.toml` file | `./inkheart.toml` |
| `BIND_ADDR` | Network address the server binds to | `127.0.0.1` (`0.0.0.0` in Docker) |
| `BIND_PORT` | HTTP port | `8080` |
| `SCAN_INTERVAL` | Filesystem scan interval for changes (in seconds) | `600` (10 min) |
| `COVER_WIDTH` | Width in pixels for extracted cover art | `900` |
| `THUMBNAIL_WIDTH` | Width in pixels for preview thumbnails | `300` |
| `TEXT_INDEX_ENABLED` | Enables full-text search indexing | `true` |
| `BASE_PATH` | Subdirectory path prefix (e.g. `/books`) when behind reverse proxy | `""` |
| `FIREBASE_CONFIG_PATH` | Path to `firebase.json` for optional Google/email auth | `./firebase.json` |
| `FIREBASE_WHITELIST` | Path to text file with whitelisted user IDs | None |

---

## 🌐 Reverse Proxy & Subdirectory Setup

To serve Inkheart behind a reverse proxy (such as Nginx, Caddy, Traefik, or Cloudflare Tunnels):

### Nginx Root Domain
```nginx
location / {
    proxy_pass http://127.0.0.1:8080;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}
```

### Subdirectory (e.g. `/library/`)
Set `BASE_PATH=/library` in your container environment, then configure Nginx:
```nginx
location /library/ {
    proxy_pass http://127.0.0.1:8080/;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}
```

---

## 🛠️ Building from Source & CI/CD

If you want to build locally or customize the code:

### Local Docker Build
```bash
docker build -t inkheart:local -f docker/dockerfile .
```

### Automated CI/CD
This repository includes an automated GitHub Actions workflow ([`.github/workflows/docker.yml`](.github/workflows/docker.yml)) that builds multi-arch images (`linux/amd64`, `linux/arm64`) on every push and publishes them directly to GitHub Container Registry (`ghcr.io`).

---

## 🏗️ Architecture

```
inkheart/
├── backend/            # Rust Actix-Web server
│   ├── indexer/        # Filesystem scanner, PDF/EPUB cover & chapter extraction
│   │   └── text_search/# Tantivy search engine, mtime change tracking, HTML parser
│   ├── server/         # REST API routes, auth extractor, static file serving
│   └── pdfium/         # Native PDFium bindings for PDF rendering
├── frontend/           # Svelte 4 SPA
│   ├── src/ui/         # NativePdf (PDF.js) & EpubReader (epub.js) components
│   └── src/stores/     # LocalStorage reading progress, themes, collections, pins
└── docker/             # Multi-stage multi-arch Dockerfile (cargo-chef + node + distroless)
```

---

## 📸 Screenshots

<p align="center">
  <img src="screenshots/folders.png" width="31%" alt="Folders">
  <img src="screenshots/files.png" width="31%" alt="Files">
  <img src="screenshots/reader.png" width="31%" alt="PDF Reader">
</p>
<p align="center">
  <img src="screenshots/reader-spread.png" width="48%" alt="Reader Spread">
  <img src="screenshots/opengraph-preview.png" width="48%" alt="Link Preview">
</p>

---

## 📄 License & Credits

- Inkheart was originally created by **[Nystik](https://github.com/Nystik-gh)** ([original repo](https://github.com/Nystik-gh/inkheart) · [support Nystik on Ko-fi](https://ko-fi.com/nystik)).
- EPUB support, multi-architecture GitHub Actions, and enhancements by **[Keshav Sapra](https://github.com/keshavsapra)**.
- Licensed under the **[Apache License, Version 2.0](LICENSE)**. You are free to use, modify, and distribute this software in accordance with the terms of the Apache 2.0 license.
- Logo and branding assets are copyright of their respective authors.
