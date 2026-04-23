---
id: goal_define_proofpack_boundary_v0_1
title: "Define proofpack boundary v0.1"
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
report_refs:
  - "work/reports/2026-04-22-proofpack-boundary-v0-1.md"
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

This goal is complete:
- `evals/specs/proofpack-boundary.v0.1.md` now defines proofpack as a future provenance and evidence bundle distinct from event, receipt, eval, and gate-decision surfaces;
- the spec keeps proofpack, `.punk/proofs`, validators, signing, storage, and CLI work deferred;
- the next conservative step is `work/goals/goal_run_fourth_work_ledger_review.md` before selecting a new implementation or process branch.
