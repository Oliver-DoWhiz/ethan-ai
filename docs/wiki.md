# Ethan AI Wiki

## What this repo is

Ethan AI is a premium Rust MVP for a narrow video-ops workflow:
real-estate listing footage in, campaign packet out.

## How to run it

```bash
PATH=/usr/local/cargo/bin:$PATH cargo run
```

Open `http://127.0.0.1:3000`.

## Demo flow

1. Open the landing page.
2. Click `Build A Campaign` or go to `/studio`.
3. Click `Load Sample`.
4. Submit the brief with `Generate Ethan Plan`.
5. Review:
   - creative direction
   - workflow
   - three template outputs
   - publish pack
   - human review notes

## Core templates

### Open House Masterpiece

- 16:9
- flagship walkthrough
- emotional reveal first, numbers later

### Hero Short

- 9:16
- one-feature hook
- quick conversion CTA

### Listing Explainer

- 4:5
- dense factual packaging
- built for reposts and warm leads

## Key product rules

- Ethan should stay narrow in scope.
- Template quality matters more than free-form generation.
- Human review stays intentionally small.
- Packaging belongs in the same workflow as the cut.

## Code map

- `src/domain.rs`: all planning logic
- `src/web.rs`: routes and APIs
- `static/landing.html`: landing
- `static/studio.html`: studio
- `static/app.js`: form submission and rendering
- `static/app.css`: visual system

## Suggested next steps

1. Add job persistence.
2. Add plan export JSON.
3. Integrate ASR and shot segmentation.
4. Add real rendering.
5. Add publishing connectors.
