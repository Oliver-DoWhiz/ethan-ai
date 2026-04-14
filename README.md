# Ethan AI

Ethan AI is a premium, local-first MVP for **template-driven real-estate video production**.

This repo delivers:

- a high-end landing page
- a campaign studio for defining a listing brief
- a Rust API that composes an edit plan and publish pack
- PRD, tech design, and operator docs aligned to the requested Ethan scope

## Product direction

Ethan is intentionally **not** framed as an infinitely creative editor.

The first version is a narrower, more operational product:

- ingest a listing brief and footage manifest
- map the footage into a reusable edit plan
- output a long-form walkthrough, a short-form cut, and a listing explainer
- generate campaign packaging: title ideas, description, chapters, thumbnails, subtitles, and export targets

## Local run

```bash
PATH=/usr/local/cargo/bin:$PATH cargo run
```

Open `http://127.0.0.1:3000`.

## Test

```bash
PATH=/usr/local/cargo/bin:$PATH cargo test
```

## Repo structure

- `src/domain.rs`: Ethan planning engine, template catalog, and output shaping
- `src/web.rs`: routes and API handlers
- `static/landing.html`: premium landing page
- `static/studio.html`: operator intake and result UI
- `static/app.css`: shared visual system
- `static/app.js`: studio interactions and API rendering
- `docs/prd.md`: product requirements doc
- `docs/tech-design.md`: system design
- `docs/wiki.md`: operator-facing wiki
- `docs/market-notes.md`: condensed research and product framing inputs

## Positioning

Ethan should feel:

- premium, editorial, and controlled
- narrow enough to be operationally credible
- modular enough to evolve into a reusable DoWhiz skill

The core bet is that **template quality and review discipline** matter more than pretending the product can improvise infinitely.
