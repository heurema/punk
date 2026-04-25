---
id: goal_add_proofpack_kernel_minimal_v0_1
title: "Add minimal proofpack kernel v0.1"
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
    - "crates/punk-proof/Cargo.toml"
    - "crates/punk-proof/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-gate/**"
    - ".punk/**"
acceptance:
  - "Proofpack kernel models a side-effect-free post-gate proofpack object with explicit proofpack id, schema version, gate decision ref, contract refs, run receipt refs, optional eval/event/output refs, created-at, and boundary notes."
  - "Proofpack kernel can record hash/reference metadata without writing `.punk/proofs`, CLI output, gate decisions, runtime storage, or acceptance claims."
  - "Proofpack remains a provenance/evidence bundle and never becomes the final decision authority."
  - "A positive acceptance claim remains unavailable unless an accepting gate decision and matching proofpack are both present."
  - "No CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "evals/specs/gate-decision-boundary.v0.1.md"
  - "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md"
  - "work/reports/2026-04-25-eleventh-work-ledger-review.md"
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
  rationale: "This will implement already-defined proofpack and proof-before-acceptance boundaries as a side-effect-free Rust kernel without runtime storage, CLI behavior, or new architecture decisions."
  research_refs:
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "evals/specs/gate-decision-boundary.v0.1.md"
    - "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md"
    - "work/reports/2026-04-25-eleventh-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md"
  rationale: "Execution will change active-core Rust proof behavior and record a work report; no product docs are expected unless behavior boundaries change."
---

## Context

Punk now has:

- minimal contract lifecycle kernel;
- flow guard and event evidence;
- run receipt kernel with minimal fields and missing-validator evidence;
- side-effect-free gate decision kernel;
- proofpack boundary v0.1;
- proof-before-acceptance semantics v0.1.

The next smallest active-core implementation step is a side-effect-free proofpack kernel that can model post-gate provenance without writing proofpack files or claiming acceptance.

## Notes

Keep this as a pure kernel change.

Do not write `.punk/proofs`.
Do not add CLI behavior.
Do not implement a proofpack writer.
Do not claim acceptance.
Do not collapse proofpack into gate decision.
Do not add adapters, automation, provider/model runners, or schema files.
