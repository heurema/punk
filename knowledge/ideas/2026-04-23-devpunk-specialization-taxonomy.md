---
id: idea_2026_04_23_devpunk_specialization_taxonomy
title: "DevPunk specialization taxonomy without top-level module explosion"
kind: idea
status: raw
authority: speculative
owner: vitaly
created_at: 2026-04-23
updated_at: 2026-04-23
review_after: 2026-07-23
components: [modules, dev-module, module-host, architecture, eval]
related_goals: []
related_files:
  - docs/product/MODULES.md
  - docs/product/MODULE-HOST.md
  - docs/product/ROADMAP.md
  - docs/product/ARCHITECTURE.md
source_refs:
  - docs/product/MODULES.md
  - docs/product/MODULE-HOST.md
  - docs/product/ROADMAP.md
confidence: medium
---

# DevPunk specialization taxonomy without top-level module explosion

## Summary

`DevPunk` may be too broad as a single undifferentiated future software-work module.

Backend, frontend, mobile, and testing work differ enough in toolchains, artifacts, evals, and bounded side effects that Punk may need a specialization model inside the future dev module.

This idea does **not** propose activating modules now. It preserves a future design option for the parked Module Host and future Dev module.

## Idea

Keep one future top-level software-work module family:

- `DevPunk`

Then specialize below that family instead of immediately creating many top-level modules.

Possible specialization layers:

1. **family**
   - `dev`
2. **lane**
   - `backend`
   - `frontend`
   - `mobile`
   - `test`
3. **stack/profile overlay**
   - `go`
   - `rust`
   - `python`
   - `node`
   - `react`
   - `ios`
   - `android`
4. **task kind**
   - `feature`
   - `bugfix`
   - `refactor`
   - `migration`
   - `test`
   - `release`

The lane/profile combination could shape module behavior without changing Punk's core lifecycle.

## What specialization should change

Specialization may later affect:

- contract templates
- scope presets
- allowed tools/capability grants
- run checklists
- receipt fields
- module assessments
- eval packs
- inspect/status hints
- proof expectations

Examples:

- `backend + go` may care about service boundaries, API contracts, migrations, build/test receipts, and deployment-risk notes.
- `frontend + react` may care about component boundaries, screenshots, browser checks, bundle checks, and UI regression evidence.
- `mobile + ios/android` may care about simulator/device flows, signing/build variants, store/distribution constraints, and platform-specific test receipts.
- `test` may become either a lane or a cross-cutting pack that can attach to any other lane.

## What specialization must not change

Specialization must not create:

- a second lifecycle
- a second event log
- a second decision authority
- lane-owned project truth
- plugin-first product claims
- top-level module sprawl before the roadmap allows it

The non-negotiables remain:

- one CLI: `punk`
- one lifecycle: `plot / cut / gate`
- only `gate` writes final decisions
- modules assess, but do not decide
- adapters invoke, but do not own truth

## Why preserve it

This idea may help later because it could:

- avoid an overly vague `DevPunk`
- reduce irrelevant checks for narrowly scoped work
- keep a single software-work family while still allowing Unix-style specialization
- support more precise eval packs and receipts
- make future module UX closer to how real software work is separated in practice

## Preferred direction

Prefer:

- one future `DevPunk` family
- internal specialization by lane/profile/pack

Avoid for now:

- `FrontPunk`, `BackPunk`, `MobilePunk`, `GoPunk`, `RustPunk`, etc. as immediate top-level product modules
- committing top-level module taxonomy before Module Host and Dev module research

## Open questions

- Should `test` be a lane, a pack, or a cross-cutting module capability?
- Should `mobile` stay inside `DevPunk`, or eventually become a separate module family?
- What exact receipt/eval differences justify a new lane?
- When does a lane/profile become important enough to promote into a separate module?
- How much specialization should be public CLI surface versus contract metadata only?

## Anti-promises

Do not claim today:

- Punk already has specialized dev modules
- DevPunk has been activated
- lane/profile taxonomy is canonical architecture
- backend/frontend/mobile are separate current modules
- stack-specific module APIs are chosen

## Promotion trigger

Revisit during:

- Phase 6 Module Host design research
- Phase 7 Dev module design work

Promotion requires:

- R1/R2 research on dev-module specialization strategy
- module taxonomy ADR
- receipt/eval implications
- conformance evals proving specialization does not bypass core laws
