---
id: goal_add_side_effect_receipt_writer_host_path_observation_model_v0_1
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

# Add Side-effect Receipt Writer Host Path Observation Model v0.1

## Purpose

Add the next smallest Module Host receipt writer prerequisite after the
target/storage policy model: a pure/no-IO host path observation model.

The model must keep host path observations explicit, redacted when sensitive,
non-authoritative, and blocked fail-closed before any local receipt writer can
be considered.

## Scope

Included:

- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_side_effect_receipt_writer_host_path_observation_model_v0_1.md`
- `work/reports/2026-05-16-side-effect-receipt-writer-host-path-observation-model-v0-1.md`

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

- Adds a pure/no-IO Module Host host path observation model.
- Consumes a ready side-effect receipt writer target/storage policy model.
- Requires explicit host path kind, optional host path ref, redaction state,
  observation blockers, and boundary notes.
- Keeps storage refs, receipt target refs, target path refs, policy refs, and
  host path refs non-authoritative.
- Blocks missing/unsafe host path refs, unredacted sensitive observations,
  ambiguous observations, and explicit path blockers fail-closed.
- Adds smoke coverage proving the model is side-effect-free.
- Updates active vs parked status in docs/status.
- Adds no runtime behavior, CLI behavior, adapters, bots, external side
  effects, or publication behavior.
- Leaves `selected_next` unchanged.

## Outcome

Done.

Implemented `ModuleSideEffectReceiptWriterHostPathObservationModel` and smoke
coverage as an incubating `punk-module-host` library model only.
