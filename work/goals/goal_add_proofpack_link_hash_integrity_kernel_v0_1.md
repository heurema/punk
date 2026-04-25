---
id: goal_add_proofpack_link_hash_integrity_kernel_v0_1
title: "Add proofpack link/hash integrity kernel v0.1"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-proof/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - ".punk/**"
acceptance:
  - "Proofpack kernel can structurally assess whether required refs have matching artifact digest entries."
  - "Integrity checks cover gate decision, contract, run receipt, and optional eval/event/output refs without computing or normalizing hashes."
  - "Missing required proofpack digests remain visible and block proof/acceptance readiness signals."
  - "Proofpack remains a provenance/evidence bundle and never becomes the final decision authority."
  - "No CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md"
  - "work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md"
  - "work/reports/2026-04-25-thirteenth-work-ledger-review.md"
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
  rationale: "This will implement proofpack link/hash integrity checks over already-defined proofpack metadata as side-effect-free Rust helpers without runtime storage, writer behavior, hash normalization, or new architecture decisions."
  research_refs:
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md"
    - "work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md"
    - "work/reports/2026-04-25-thirteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md"
  rationale: "Execution will change active-core proof kernel behavior and record a work report; no product docs are expected unless behavior boundaries change."
---

## Context

Punk now has:

- side-effect-free gate decision authority;
- side-effect-free proofpack provenance metadata;
- deterministic smoke eval coverage for the acceptance chain.

The next smallest active-core step is to make proofpack ref/hash consistency inspectable in the proof kernel before any writer, storage, CLI, or orchestration work.

## Notes

Keep this as structural integrity checking only.

Do not compute artifact hashes.
Do not define hash normalization.
Do not write `.punk/proofs`.
Do not add CLI behavior.
Do not implement a proofpack writer.
Do not claim acceptance.
Do not collapse proofpack into gate decision.
Do not add adapters, automation, provider/model runners, or schema files.
