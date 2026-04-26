---
id: goal_define_proofpack_manifest_digest_boundary_v0_1
title: "Define proofpack manifest digest boundary v0.1"
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
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "Proofpack manifest digest boundary v0.1 is defined as a docs/spec artifact before implementation."
  - "The boundary states that any future helper hashes the exact UTF-8 bytes of the deterministic in-memory proofpack manifest rendering."
  - "The boundary distinguishes proofpack manifest self-digest from referenced artifact digests and from file IO artifact hashing."
  - "The boundary defines how to avoid overclaiming `punk-proof` hash computation, hash verification, proofpack writer behavior, runtime storage, gate decisions, acceptance claims, or CLI behavior."
  - "The boundary records that `punk-core` owns exact-byte SHA-256 computation and no new hash dependency should be added to `punk-proof` for this helper."
  - "The boundary names minimal future tests/evals for deterministic manifest digest behavior without claiming runtime proof or acceptance."
  - "No runtime/code/schema/CLI/`.punk` changes are made."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "crates/punk-proof/src/lib.rs"
  - "crates/punk-core/src/lib.rs"
  - "work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md"
  - "work/reports/2026-04-26-twenty-fifth-work-ledger-review.md"
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
  rationale: "This is a bounded docs/spec decision for a future proofpack manifest digest helper, using existing Punk docs, specs, and code artifacts; no Deep Research is needed unless a major architecture conflict appears."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-core/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md"
    - "work/reports/2026-04-26-twenty-fifth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md"
  rationale: "The selected work defines a narrow eval/spec boundary for future proofpack manifest digest computation before implementation."
---

## Context

`punk-proof` can render a deterministic in-memory proofpack manifest.
`punk-core` can compute canonical SHA-256 digests from exact caller-provided bytes.

The next safe seam is defining how a future proofpack manifest digest helper should combine these capabilities without becoming a proofpack writer, file IO hasher, runtime storage layer, gate decision writer, acceptance claim, schema change, or CLI behavior.

## Notes

Keep this docs/spec-only.

Do not edit crates.
Do not add dependencies.
Do not compute hashes in code.
Do not write files.
Do not write `.punk` state.
Do not add CLI behavior.
Do not add schema files.
Do not implement proofpack writer behavior.
Do not add file IO hashing.
Do not write gate decisions.
Do not create acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
