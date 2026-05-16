---
id: goal_add_side_effect_receipt_writer_target_storage_policy_model_v0_1
kind: goal
status: done
authority: canonical
owner: vitaly
created_at: 2026-05-16
updated_at: 2026-05-16
module: module-host
priority: P1
research_gate:
  classification: R2
  required: true
  rationale: "Small Runtime Automation Spine implementation slice derived from existing Module Host and proofpack writer boundaries; no external research required."
---

# Add Side-effect Receipt Writer Target/Storage Policy Model v0.1

## Purpose

Add the next smallest Module Host receipt writer prerequisite after the file IO
plan model: a pure/no-IO target/storage policy readiness model.

The model must keep storage refs, receipt target refs, target path refs, policy
refs, idempotency/conflict policy, temp/atomic policy, and
operation-evidence-persistence policy explicit before any local receipt writer
can be considered.

## Scope

Included:

- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_side_effect_receipt_writer_target_storage_policy_model_v0_1.md`
- `work/reports/2026-05-16-side-effect-receipt-writer-target-storage-policy-model-v0-1.md`

Excluded:

- public CLI expansion
- Module Host runtime
- plugin loading
- module execution
- module manifests
- Wasm or Extism runtime
- active receipt writer
- receipt file writes
- filesystem reads or writes
- host path resolution
- runtime storage
- `.punk/**` runtime writes
- persisted operation evidence
- event-log mutation
- policy engine invocation
- gate invocation
- gate decision writing
- proofpack writing
- adapter invocation
- PubPunk runtime
- CommunityPunk runtime
- publishing, commenting, issue creation, or PR creation
- browser/API calls
- credential access
- bots/adapters/integrations
- DAO/token/funding behavior

## Acceptance

- Adds a pure/no-IO Module Host target/storage policy readiness model.
- Consumes a ready side-effect receipt writer file IO plan.
- Requires explicit storage, receipt target, target path, and policy refs.
- Keeps storage refs, receipt target refs, target path refs, and policy refs
  non-authoritative.
- Blocks missing/unsafe policy refs and missing failure visibility fail-closed.
- Adds smoke coverage proving the model is side-effect-free.
- Updates active vs parked status in docs/status.
- Adds no runtime behavior, CLI behavior, adapters, bots, external side
  effects, or publication behavior.
- Leaves `selected_next` unchanged.

## Outcome

Done.

Implemented `ModuleSideEffectReceiptWriterTargetStoragePolicyModel` and smoke
coverage as an incubating `punk-module-host` library model only.
