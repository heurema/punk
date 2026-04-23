---
id: goal_define_proofpack_boundary_v0_1
title: "Define proofpack boundary v0.1"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-23
updated_at: 2026-04-23
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "evals/specs/**"
    - "work/**"
    - "knowledge/research/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "A design/spec artifact defines the future proofpack boundary without implementing proofpack, gate, or `.punk/proofs`."
  - "The proposal states how proofpack differs from gate decision, run receipt, eval report, and event evidence."
  - "The proposal defines a minimum proofpack metadata surface and explicit non-goals."
  - "No Rust code, `.punk/`, validators, or proofpack implementation is introduced."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/PUNK-LAWS.md"
  - "knowledge/research/2026-04-22-proofpack-boundary.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "R2 research on the proofpack boundary is complete. The next bounded step should be a design/spec artifact only, not proofpack or gate implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "knowledge/research/2026-04-22-proofpack-boundary.md"
  external_research_refs:
    - "work/reports/2026-04-22-proofpack-boundary-research.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Proofpack boundary research is complete. The next honest step is to define a boundary/spec artifact before any proofpack, `.punk/proofs`, validator, or gate/proof implementation begins.

## Notes

Keep this goal narrow:
- design/spec only;
- no Rust code;
- no `.punk/`;
- no proofpack implementation;
- no validators.
