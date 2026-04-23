---
id: goal_define_gate_decision_boundary_v0_1
title: "Define gate decision boundary v0.1"
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
  - "A design/spec artifact defines the future gate decision boundary without implementing gate, proof, or `.punk/decisions`."
  - "The proposal states how gate decision differs from event evidence, run receipt, eval report, and proofpack."
  - "The proposal defines a minimum DecisionObject surface and explicit non-goals."
  - "No Rust code, `.punk/`, validators, or gate/proof implementation is introduced."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/PUNK-LAWS.md"
  - "knowledge/research/2026-04-22-gate-decision-boundary.md"
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
  rationale: "R2 research on the gate decision boundary is complete. The next bounded step should be a design/spec artifact only, not gate or proof implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "knowledge/research/2026-04-22-gate-decision-boundary.md"
  external_research_refs:
    - "work/reports/2026-04-22-gate-decision-boundary-research.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Gate decision research is complete. The next honest step is to define a boundary/spec artifact before any gate, proof, `.punk/decisions`, or validator implementation begins.

## Notes

Keep this goal narrow:
- design/spec only;
- no Rust code;
- no `.punk/`;
- no gate implementation;
- no proofpack implementation;
- no validators.
