---
id: goal_define_artifact_hash_computation_helper_boundary_v0_1
title: "Define artifact hash computation helper boundary v0.1"
status: ready
owner: "vitaly"
module: "core"
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
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "Artifact hash computation helper boundary v0.1 is defined as a docs/spec artifact before implementation."
  - "The boundary states that future hash computation consumes exact caller-provided bytes and emits canonical `sha256:<64 lowercase hex>` digest metadata."
  - "The boundary explicitly decides the dependency stance for a future Rust SHA-256 implementation before code changes."
  - "The boundary keeps path/ref validation separate from byte hashing and does not introduce byte normalization, path normalization, file IO, runtime storage, schema files, CLI behavior, proofpack writing, gate decisions, adapters, automation, or `punk init`."
  - "The boundary names minimal future tests/eval cases for deterministic byte hashing without claiming runtime proof or acceptance."
  - "No runtime/code/schema/CLI/`.punk` changes are made."
knowledge_refs:
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "work/reports/2026-04-26-twenty-second-work-ledger-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded docs/spec decision for the future active hash computation helper, including API and dependency stance, using existing Punk docs, specs, and repo manifests; no Deep Research is needed unless a major architecture conflict appears."
  research_refs:
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-twenty-second-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: eval-spec
  required_updates:
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md"
  rationale: "The selected work defines a narrow eval/spec boundary for future hash computation before implementation."
---

## Context

Punk now has artifact hash policy validation helpers and a deterministic side-effect-free proofpack manifest renderer.

The next risky seam is active SHA-256 computation: Rust stdlib does not provide a ready hash helper, while proofpack writer/runtime work should not depend on ad hoc digest generation.

Before adding code or dependencies, define the helper boundary and dependency stance explicitly.

## Notes

Keep this docs/spec-only.

Do not edit crates.
Do not compute hashes.
Do not add dependencies.
Do not write files.
Do not write `.punk/` state.
Do not add CLI behavior.
Do not add schema files.
Do not implement proofpack writer behavior.
Do not write gate decisions.
Do not create acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
