# Ethan AI Technical Design

## Document intent

This design doc translates the Ethan PRD into a working MVP architecture.
The structure follows a compact technical-design pattern: context, goals,
modules, APIs, data shapes, testing, and future evolution.

## Context

The requested MVP needs to:

- run locally
- feel premium in the browser
- stay close to DoWhiz’s Rust preference
- expose a credible middle layer for future rendering work

## Goals

- ship a local product with landing page plus studio
- keep the logic modular and testable
- encode Ethan’s template system directly in Rust domain code
- make future AI/render integrations additive rather than rewriting the app

## Non-goals

- production auth
- persistence
- actual media processing
- real model calls
- actual publishing to third-party platforms

## Architecture overview

Ethan AI is a small Rust web app built with:

- `axum` for routes and JSON APIs
- `tower-http` for static assets
- static HTML/CSS/JS for the premium frontend
- pure Rust domain logic for planning outputs

## System diagram

```text
Browser
  -> GET /               -> landing.html
  -> GET /studio         -> studio.html
  -> GET /api/templates  -> template catalog JSON
  -> POST /api/plan      -> planning engine
  -> GET /static/*       -> CSS / JS
```

## Module layout

### `src/domain.rs`

Contains:

- Ethan request/response models
- template catalog
- planning engine
- review-note logic
- publish-pack generation

### `src/web.rs`

Contains:

- application router
- HTML routes
- API handlers
- static serving

### `static/`

Contains:

- landing page
- studio page
- shared premium CSS system
- studio interaction logic

## Data shapes

### `GenerateRequest`

- listing metadata
- buyer and creative inputs
- room sequence
- asset manifest

### `ProjectResponse`

- `creative_direction`
- `workflow`
- `templates[]`
- `publish_pack`
- `review_notes[]`

### `TemplateOutput`

- template identity
- duration and aspect ratio
- opening hook
- summary
- `shot_plan[]`
- `overlays[]`

## Planning pipeline

1. Validate required input fields.
2. Normalize room names and listing facts.
3. Build a creative-direction layer.
4. Build workflow steps.
5. Generate three output templates:
   - Open House Masterpiece
   - Hero Short
   - Listing Explainer
6. Generate publish-pack assets.
7. Generate review notes based on asset gaps and room coverage.

## Why this design

- keeps Ethan’s core value in a reusable middle layer
- avoids hiding logic inside frontend-only code
- makes unit testing straightforward
- supports later replacement of deterministic copy with model-generated copy

## API design

### `GET /api/templates`

Returns the high-level template catalog so the frontend can show the campaign
shape immediately on load.

### `POST /api/plan`

Accepts a `GenerateRequest` and returns a full `ProjectResponse`.

Validation failures return `400` with a readable error message.

## Frontend design

### Landing

- premium editorial hero
- clear articulation of Ethan’s narrow product boundary
- template system and CTA

### Studio

- left-side intake console
- right-side output packet
- quick sample loading for demos
- no frontend build step

## Testing strategy

### Unit tests

- domain generation returns all templates
- domain generation rejects empty room sequences

### Integration tests

- landing route returns `200`
- studio route returns `200`
- template API returns `200`
- plan API accepts a valid payload

### Browser verification

- render landing page
- load sample listing in studio
- submit the form
- verify campaign packet is visible
- capture screenshots for delivery evidence

## Tradeoffs

1. The MVP simulates the planning layer rather than truly rendering video.
2. Asset counts and toggles stand in for persisted uploads.
3. Copy is deterministic and handcrafted instead of model-generated.

Those tradeoffs are acceptable because the first deliverable is an operational
and visual proof-of-concept, not a production video backend.

## Future evolution

### Near term

- persistent jobs
- real uploads
- editable template definitions
- export of structured plan JSON files

### Medium term

- ASR and vision integrations
- FFmpeg or timeline-render pipeline
- real subtitle generation
- reviewer comments and approval state

### Long term

- batch production queues
- publishing integrations
- analytics feedback loops
- Ethan as a reusable DoWhiz skill
