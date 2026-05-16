---
id: goal_add_side_effect_receipt_writer_operation_evidence_write_slice_v0_1
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
  rationale: "Small active-core Runtime Automation Spine slice based on already captured Module Host receipt-writer prerequisites; no external side effects, adapter behavior, or CLI behavior."
---

# Goal: Add Module Host side-effect receipt writer operation-evidence write slice v0.1

## Purpose

Add the next smallest local-only active receipt-writer slice after the first
active receipt byte write: persist exact caller-provided operation-evidence
bytes for a successful receipt-writer result under an explicit `.punk/runs`
target.

This moves Punk toward self-running local workflows without activating module
runtime, adapters, publishing, commenting, PR creation, gate decisions, or
proofpack writing.

## Scope

Include:

- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/STATUS.md`
- `work/goals/goal_add_side_effect_receipt_writer_operation_evidence_write_slice_v0_1.md`
- `work/reports/2026-05-16-side-effect-receipt-writer-operation-evidence-write-slice-v0-1.md`

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
- policy engine invocation
- gate invocation
- receipt serialization
- operation-evidence serialization
- gate decision writing
- proofpack writing
- acceptance claims
- DAO/token/funding behavior

## Acceptance

- Adds a local-only operation-evidence write slice for receipt-writer results.
- Requires a successful first active receipt-writer result before writing.
- Requires exact caller-provided operation-evidence bytes.
- Requires an explicit storage root path and explicit target-relative path under
  `.punk/runs/`.
- Requires the target-relative path to match the result operation-evidence ref.
- Uses create-new/no-overwrite behavior.
- Treats identical existing bytes as idempotent.
- Blocks different existing bytes as conflicts.
- Does not create parent directories.
- Does not mutate event logs.
- Does not invoke adapters, policy engines, gate, APIs, browser, credentials,
  publishing, comments, pull requests, proofpacks, or acceptance claims.
- Adds smoke coverage.
- Updates docs/status honestly.
- Leaves `selected_next` unchanged.

## Outcome

Done. `punk-module-host` now exposes a first active local operation-evidence
write slice for receipt-writer results. It writes exact caller-provided
operation-evidence bytes only to an explicit `.punk/runs` target under an
explicit storage root after a successful receipt write and returns
non-authoritative result evidence. Runtime/module/adapters/publishing remain
inactive.
