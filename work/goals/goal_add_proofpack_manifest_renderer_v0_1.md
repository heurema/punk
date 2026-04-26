---
id: goal_add_proofpack_manifest_renderer_v0_1
title: "Add proofpack manifest renderer v0.1"
status: ready
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "evals/specs/**"
acceptance:
  - "`punk-proof` can render an existing `Proofpack` into deterministic side-effect-free manifest content."
  - "The renderer includes proofpack id, schema version, gate decision ref, contract refs, run receipt refs, eval refs, event refs, output artifact refs, artifact digests, created_at, and boundary notes."
  - "The rendered representation is stable for identical inputs and has tests for escaping strings that need escaping."
  - "The renderer preserves existing proofpack validation and artifact hash policy validation; it does not accept non-canonical proof artifact hashes."
  - "The renderer does not write proofpacks, compute hashes, normalize bytes or hashes, add runtime storage, add schema files, add CLI behavior, write gate decisions, create acceptance claims, add adapters, add automation, or implement `punk init`."
  - "Smoke eval output may mention renderer coverage only as local assessment, not as final acceptance or runtime proofpack writing."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
knowledge_refs:
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "work/reports/2026-04-26-twenty-first-work-ledger-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This is a narrow side-effect-free implementation over the existing proofpack data model and already-defined proofpack/artifact-hash boundaries; no external research or architecture change is required."
  research_refs:
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-twenty-first-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-proofpack-manifest-renderer-v0-1.md"
  rationale: "Execution will add side-effect-free proofpack renderer behavior and record a work report; canonical docs/specs should not change unless implementation reveals a mismatch."
---

## Context

`punk-proof` now has a side-effect-free proofpack model, structural link/hash integrity checks, canonical artifact digest shape validation, and reconciled CRATE-STATUS wording.

Before a proofpack writer, runtime storage, active hash computation, byte normalization, schema file, CLI behavior, or gate/eval/proof orchestration, the proofpack model needs a deterministic side-effect-free manifest representation.

## Notes

Keep this as an in-memory rendering helper only.

Do not write files.
Do not write `.punk/` state.
Do not add CLI behavior.
Do not add schema files.
Do not implement proofpack writer behavior.
Do not compute hashes.
Do not normalize bytes or hashes.
Do not write gate decisions.
Do not create acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
