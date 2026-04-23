---
id: goal_define_gate_decision_boundary_v0_1
title: "Define gate decision boundary v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-23
updated_at: 2026-04-23
selected_at: 2026-04-23
started_at: 2026-04-23
completed_at: 2026-04-23
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
report_refs:
  - "work/reports/2026-04-22-gate-decision-boundary-v0-1.md"
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

This goal is complete:
- `evals/specs/gate-decision-boundary.v0.1.md` now defines gate decision as a future closure-authority artifact distinct from contract, flow, event, receipt, eval, and proof surfaces;
- the spec keeps gate, proofpack, validators, storage, and CLI work deferred;
- the next conservative step is research-first on the proofpack boundary before any gate or proof implementation branch.
