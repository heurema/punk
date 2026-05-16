---
id: goal_add_local_receipt_evidence_event_handoff_v0_1
kind: goal
status: done
authority: canonical
owner: vitaly
module: events
priority: P1
created_at: 2026-05-16
updated_at: 2026-05-16
completed_at: 2026-05-16
research_gate:
  classification: R2
  required: true
  rationale: "Small active-core Runtime Automation Spine slice based on existing local event and receipt/evidence writer slices; no external side effects, adapter behavior, or CLI behavior."
---

# Goal: Add local receipt/evidence event handoff v0.1

## Purpose

Add the next smallest Runtime Automation Spine link after local receipt and
operation-evidence write slices: record a local event-log handoff that links a
safe receipt ref and safe operation-evidence ref.

This moves Punk toward inspectable self-running local workflows without
activating publishing, commenting, PR creation, adapters, policy engines, gate
decisions, proofpacks, or final acceptance.

## Scope

Include:

- `crates/punk-events/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/STATUS.md`
- `work/goals/goal_add_local_receipt_evidence_event_handoff_v0_1.md`
- `work/reports/2026-05-16-local-receipt-evidence-event-handoff-v0-1.md`

Exclude:

- public CLI behavior
- persisted flow state
- event replay/state reader
- receipt or operation-evidence serialization
- receipt or operation-evidence file writing
- Module Host runtime
- plugin loading
- module execution
- PubPunk runtime
- CommunityPunk runtime
- adapters
- publishing/commenting/PR automation
- GitHub API calls
- policy engine invocation
- gate invocation
- gate decision writing
- proofpack writing
- acceptance claims
- DAO/token/funding behavior

## Acceptance

- Adds a local-only `punk-events` helper for receipt/evidence handoff events.
- Appends only to `.punk/events/flow.jsonl` under an explicit initialized
  project root.
- Links safe, distinct receipt and operation-evidence refs.
- Rejects unsafe or collapsed refs fail-closed before creating event storage.
- Does not write `.punk/runs` receipt or operation-evidence artifacts.
- Does not expose new CLI behavior.
- Does not invoke adapters, policy engines, gate, APIs, browser, credentials,
  publishing, comments, pull requests, proofpacks, or acceptance claims.
- Adds smoke coverage.
- Updates docs/status honestly.
- Leaves `selected_next` unchanged.

## Outcome

Done. `punk-events` now exposes a local receipt/evidence handoff helper that
appends a `receipt_evidence_handoff` event to `.punk/events/flow.jsonl` with
safe distinct receipt and operation-evidence refs. It writes event evidence
only; receipt/evidence artifacts, runtime state readers, CLI behavior, module
runtime, adapters, publishing, gate decisions, proofpacks, and acceptance remain
inactive.
