---
id: goal_add_side_effect_receipt_writer_concrete_path_storage_policy_model_v0_1
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
  rationale: "Small Runtime Automation Spine implementation slice derived from existing Module Host receipt-writer models and proofpack writer boundaries; no external research required."
---

# Add Side-effect Receipt Writer Concrete Path Storage Policy Model v0.1

## Purpose

Add the next smallest Module Host receipt writer prerequisite after the host path
observation model: a pure/no-IO concrete path/storage policy readiness model.

The model must compose target/storage policy evidence with host path observation
evidence before any local receipt writer can be considered. It must stay
advisory, fail closed, local-only, and side-effect-free.

## Scope

Included:

- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_side_effect_receipt_writer_concrete_path_storage_policy_model_v0_1.md`
- `work/reports/2026-05-16-side-effect-receipt-writer-concrete-path-storage-policy-model-v0-1.md`

Excluded:

- public CLI expansion
- Module Host runtime
- plugin loading
- module execution
- module manifests
- Wasm or Extism runtime
- active receipt writer
- receipt file writes
- filesystem reads, writes, inspection, resolution, or canonicalization
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

- Adds a pure/no-IO Module Host concrete path/storage policy readiness model.
- Consumes a ready side-effect receipt writer target/storage policy model.
- Consumes a ready side-effect receipt writer host path observation model.
- Requires selected policy refs, separated refs, redacted sensitive host path
  observations, and boundary notes.
- Keeps storage refs, receipt target refs, target path refs, policy refs, and
  host path refs non-authoritative.
- Blocks unready policy or observation inputs, ref mismatches, missing refs,
  unselected policy refs, unavailable or unsafe host path refs, unredacted
  sensitive host path refs, host path observations that imply receipt
  availability, and missing boundary notes fail-closed.
- Adds smoke coverage proving the model is side-effect-free.
- Updates active vs parked status in docs/status.
- Adds no runtime behavior, CLI behavior, adapters, bots, external side
  effects, or publication behavior.
- Leaves `selected_next` unchanged.

## Outcome

Done.

Implemented `ModuleSideEffectReceiptWriterConcretePathStoragePolicyModel` and
smoke coverage as an incubating `punk-module-host` library model only.
