---
id: 2026-05-16-local-receipt-evidence-event-handoff-v0-1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-16
goal_ref: work/goals/goal_add_local_receipt_evidence_event_handoff_v0_1.md
---

# Report: Local receipt/evidence event handoff v0.1

## Summary

Implemented a local-only `punk-events` receipt/evidence handoff helper. The
helper builds and appends a `receipt_evidence_handoff` event to
`.punk/events/flow.jsonl` under an explicit initialized project root, carrying
safe distinct receipt and operation-evidence refs as event artifact refs.

This is a Runtime Automation Spine step only. It does not write receipt or
operation-evidence artifacts, activate persisted flow/run state, expose new CLI
behavior, invoke adapters, publish/comment/create PRs, call GitHub APIs, invoke
gate, write proofpacks, or claim acceptance.

## Files changed

- `crates/punk-events/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/STATUS.md`
- `work/goals/goal_add_local_receipt_evidence_event_handoff_v0_1.md`
- `work/reports/2026-05-16-local-receipt-evidence-event-handoff-v0-1.md`

## Validation run

Validation was run after implementation and docs/status update. Results:

- `cargo fmt --check`: PASS
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo test -p punk-events`: PASS
- `cargo test -p punk-eval`: PASS
- `cargo run -q -p punk-cli -- eval run smoke`: PASS
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files README.md crates/punk-eval/src/lib.rs crates/punk-events/src/lib.rs docs/product/CRATE-STATUS.md docs/product/PROJECT-MEMORY.md docs/product/START-HERE.md work/STATUS.md work/goals/goal_add_local_receipt_evidence_event_handoff_v0_1.md work/reports/2026-05-16-local-receipt-evidence-event-handoff-v0-1.md --report work/reports/2026-05-16-local-receipt-evidence-event-handoff-v0-1.md`: PASS
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds a local-only event-log handoff helper for receipt/evidence refs and smoke coverage."
  touched_surfaces:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PROJECT-MEMORY.md
    - work/STATUS.md
    - work/goals/goal_add_local_receipt_evidence_event_handoff_v0_1.md
    - work/reports/2026-05-16-local-receipt-evidence-event-handoff-v0-1.md
  required_updates:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PROJECT-MEMORY.md
    - work/STATUS.md
```

Notes:

- `punk-events` remains local-only event evidence.
- The new helper writes only the event log and does not write `.punk/runs`
  artifacts.
- No external side effects, adapter invocation, publishing, commenting, PR
  creation, gate decision, proofpack writing, or acceptance claim were
  activated.

## Knowledge impact

- Canonical product docs changed: README, Start Here, Crate Status, Project
  Memory.
- Active runtime scope changed narrowly: local event logs can now record a
  receipt/evidence ref handoff event.
- Active CLI scope unchanged.
- `.punk/runs` writer scope unchanged.
- Module Host runtime remains inactive.
- PubPunk runtime remains inactive.
- CommunityPunk runtime remains inactive.
- Adapters remain inactive.
- Automated publishing/commenting/PR creation remain inactive.
- Gate/proof authority unchanged.
- DAO/token/funding remain parked/avoided.

## Drift observed

- `selected_next` still points at the older pause goal. The maintainer is
  explicitly driving a Runtime Automation Spine side-track, so this patch
  records the side-track and leaves `selected_next` unchanged.

## Out of scope

- persisted flow state
- event replay/state reader
- receipt serialization
- operation-evidence serialization
- receipt or operation-evidence file writes
- Module Host runtime
- module loading/execution
- module manifests
- Wasm/Extism runtime
- PubPunk runtime
- CommunityPunk runtime
- adapters
- external publishing
- comments
- GitHub API calls
- issue/PR automation
- policy engine invocation
- gate invocation
- gate decision writing
- proofpack writing
- acceptance claims
- product-truth authority

## Next code slice

Add replay-backed local flow/run state reading over `.punk/events/flow.jsonl`, or
add a receipt/evidence index model, before any adapter invocation or external
publishing behavior.
