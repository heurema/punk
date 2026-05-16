---
id: goal_add_side_effect_receipt_writer_first_active_write_slice_v0_1
kind: goal
status: done
authority: canonical
owner: vitaly
module: module-host
priority: P1
created_at: 2026-05-16
updated_at: 2026-05-16
completed_at: 2026-05-16
research_gate:
  classification: R2
  required: true
  rationale: "Small active-core Runtime Automation Spine slice based on already captured Module Host receipt-writer prerequisites; no external side effects or adapter behavior."
---

# Goal: Add Module Host side-effect receipt writer first active write slice v0.1

## Purpose

Add the smallest local-only active receipt-writer implementation slice after the
Module Host receipt writer preflight, active-behavior, file-IO plan,
target/storage policy, host-path observation, concrete path/storage policy, and
operation-evidence persistence readiness models.

This moves Punk toward self-running local workflows without activating module
runtime, adapters, publishing, commenting, PR creation, gate decisions, or
proofpack writing.

## Scope

Include:

- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_side_effect_receipt_writer_first_active_write_slice_v0_1.md`
- `work/reports/2026-05-16-side-effect-receipt-writer-first-active-write-slice-v0-1.md`

Exclude:

- public CLI behavior
- Module Host runtime
- plugin loading
- module execution
- PubPunk runtime
- CommunityPunk runtime
- adapters
- publishing/commenting/PR automation
- GitHub API calls
- event-log mutation
- operation-evidence persistence
- gate decision writing
- proofpack writing
- acceptance claims
- DAO/token/funding behavior

## Acceptance

- Adds a local-only first active write slice for side-effect receipt bytes.
- Requires a ready operation-evidence persistence readiness model before write.
- Requires exact caller-provided receipt bytes.
- Requires an explicit storage root path and explicit target-relative path under
  `.punk/runs/`.
- Uses create-new/no-overwrite behavior.
- Treats identical existing bytes as idempotent.
- Blocks different existing bytes as conflicts.
- Does not create parent directories.
- Does not persist operation evidence.
- Does not mutate event logs.
- Does not invoke adapters, policy engines, gate, APIs, browser, credentials,
  publishing, comments, pull requests, proofpacks, or acceptance claims.
- Adds smoke coverage.
- Updates docs/status honestly.
- Leaves `selected_next` unchanged.

## Outcome

Done. `punk-module-host` now exposes a first active local side-effect receipt
writer slice that writes exact caller-provided bytes only to an explicit
`.punk/runs` target under an explicit storage root and returns in-memory
non-authoritative operation evidence. Smoke coverage writes only under a
temporary project root. Runtime/module/adapters/publishing remain inactive.
