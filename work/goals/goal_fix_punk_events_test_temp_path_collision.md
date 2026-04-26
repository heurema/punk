---
id: goal_fix_punk_events_test_temp_path_collision
title: "Fix punk-events test temp path collision"
status: done
owner: "vitaly"
module: "events"
priority: P0
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: 2026-04-26
completed_at: 2026-04-26
blocked_by: []
scope:
  include:
    - "crates/punk-events/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - ".punk/**"
acceptance:
  - "`punk-events` file-backed test temp paths remain unique when tests run concurrently."
  - "Fix is limited to test helper uniqueness and does not change event log runtime behavior or public API."
  - "No CLI commands, schemas, `.punk/` storage, gate/proof writer, proofpack writer, provider/model/agent adapter, automation, or `punk init` is added."
  - "`cargo test --workspace` passes after the fix."
knowledge_refs:
  - "work/reports/2026-04-26-eighteenth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-punk-events-test-temp-path-collision.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "Validation uncovered a deterministic local test helper collision risk; the fix is a small repo-local test-only reliability patch with no architecture or runtime behavior change."
  research_refs:
    - "crates/punk-events/src/lib.rs"
    - "work/reports/2026-04-26-eighteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-punk-events-test-temp-path-collision.md"
  rationale: "Code behavior changed only in test helper uniqueness; record the validation blocker and fix in a work report."
---

## Context

Repeated `cargo test --workspace` validation runs exposed a `punk-events` file-backed test collision.
The test helper built temp paths from process id and wall-clock nanoseconds; concurrent tests can observe the same timestamp granularity on the same process and reuse the same file path.

## Outcome

Added an atomic counter to the `punk-events` test temp path helper so concurrently running tests get distinct paths even when the clock value collides.

No product runtime behavior changed.
No CLI surface changed.
No schema or `.punk/` storage was added.
