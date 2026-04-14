# Ethan AI PRD

Version: v1  
Date: 2026-04-14  
Owner: Oliver  
Status: MVP delivered

## 1. Document intent

This PRD defines the first product direction for **Ethan AI**, a premium
video-editing skill agent focused on a narrow and operational use case:
turning raw real-estate footage into a publish-ready campaign.

The structure follows strong public PRD patterns: intent, problem, users,
scope, requirements, success metrics, risks, and launch framing.

## 2. Executive summary

Ethan AI is a **template-driven video production operator** for agent-native
real-estate teams.

Its first promise is deliberately narrow:

- ingest raw open-house footage and listing context
- compose a reusable edit plan
- package one flagship walkthrough plus supporting short-form outputs
- produce titles, description, thumbnail copy, subtitles, and export targets

The first version should not pretend to be a full autonomous creative director.
It should feel premium, fast, and operationally trustworthy.

## 3. Product vision

### Vision

Build the most tasteful and operationally credible AI video operator for
high-end real-estate teams.

### Long-term ambition

Start with one narrow wedge. Later expand into a broader DoWhiz video skill
that other agents can call for:

- campaign packaging
- batch production
- publishing workflows
- analytics-informed iteration

## 4. Problem

Real-estate teams already capture more raw footage than they can package well.
The main failures are predictable:

1. Editing is slow and expensive.
2. Short-form and long-form outputs drift apart.
3. Marketing metadata gets written as an afterthought.
4. Premium listings often get generic social-video treatment.

The opportunity is to create a system that turns one source packet into a
coherent campaign.

## 5. Positioning

### Positioning statement

For agent-native real-estate teams that need premium video output without
creative chaos, **Ethan AI** is a template-driven video operator that converts
raw footage into controlled campaigns, not just isolated edits.

### What makes Ethan different

- narrower scope than generic video-AI products
- premium editorial tone instead of growth-hack aesthetics
- an explicit middle layer: the edit plan
- packaging output included as part of the product, not a manual tail task

## 6. Product principles

1. **Operational before magical**  
   The product should feel trustworthy before it feels expansive.

2. **Template quality is the moat**  
   Reusable campaign structures matter more than free-form generation.

3. **High-end, not loud**  
   The interface and copy should feel deliberate and expensive.

4. **Review is a feature**  
   Human touchpoints must be narrow and intentional.

5. **Package the campaign, not just the cut**  
   Titles, thumbnails, description, chapters, and subtitle strategy belong in the same workflow.

## 7. Target users

### Primary audience

- real-estate agents and small teams producing listing walkthroughs
- operator-minded founders who need systemized video output
- high-end property marketers who care about tone as much as throughput

### Secondary audience

- broker teams with in-house content coordinators
- operators running batch content workflows across multiple listings

### Explicitly out of scope for v1

- general creator workflows
- weddings, podcasts, ecommerce, and every other video vertical
- advanced effect-heavy editing
- autonomous publishing without review

## 8. Core use case

### Ethan’s first wedge

**Open house walkthrough video**

Inputs:

- phone-shot listing footage
- listing facts
- photo pack
- optional voice notes
- brand voice selection

Outputs:

- 1 long-form YouTube-ready walkthrough
- 1 short-form hero cut
- 1 dense listing explainer
- title candidates
- description
- chapter suggestions
- thumbnail copy
- subtitle strategy

## 9. MVP scope

### Included

- premium landing page
- studio/intake page
- template catalog
- planning engine
- long-form, short-form, and explainer output definitions
- publish pack generation
- local runnable Rust application
- documentation and tests

### Excluded

- persistent storage
- account system
- actual media upload persistence
- real FFmpeg rendering
- real ASR / vision inference
- publishing API integration

## 10. Functional requirements

1. Users can enter listing facts and an asset manifest.
2. Users can choose the rooms that define the narrative spine.
3. Users can set brand voice and hook style.
4. Ethan returns a structured campaign packet with:
   - creative direction
   - operator workflow
   - per-template shot plan
   - overlay list
   - publish pack
   - review notes

## 11. UX direction

Ethan should look like a luxury operator console rather than a consumer social
tool:

- editorial serif display typography
- restrained warm neutrals with a rust accent
- deliberate spacing
- premium glass/paper surfaces
- no dashboard clutter and no generic AI gradients

## 12. Success metrics

### Product metrics

- users can complete a brief in under 3 minutes
- output is understandable without extra explanation
- each template reads as distinct and intentional

### Operational metrics for future versions

- time from ingest to first reviewable cut
- human edit minutes per listing
- factual-error rate
- clip-repetition rate
- CTR / watch-time lift after publishing

## 13. Risks

1. If Ethan becomes too broad, it loses operational credibility.
2. If template quality is weak, the whole product collapses into generic output.
3. If review boundaries are vague, the workflow becomes expensive again.
4. If the product overclaims automation before real rendering exists, trust erodes.

## 14. Launch framing

The MVP should be shown as:

- a serious planning and packaging layer
- a premium frontend with a convincing operator workflow
- a foundation for future rendering and publishing integrations
