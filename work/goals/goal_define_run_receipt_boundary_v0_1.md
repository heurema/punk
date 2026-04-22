---
id: goal_define_run_receipt_boundary_v0_1
title: "Define run receipt boundary v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
selected_at: 2026-04-22
started_at: 2026-04-22
completed_at: 2026-04-22
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
  - "A design/spec artifact defines the future run receipt boundary without implementing `.punk/runs` or receipt schemas."
  - "The proposal states how receipt differs from event log, eval report, gate decision, and proofpack."
  - "The proposal defines minimum boundary fields and explicit non-goals."
  - "No Rust code, `.punk/`, receipt validators, or gate/proof implementation is introduced."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "knowledge/research/2026-04-22-run-receipt-boundary.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-run-receipt-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "R2 research on run receipt boundaries is complete. The next bounded step should be a design/spec artifact only, not implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "knowledge/research/2026-04-22-run-receipt-boundary.md"
  external_research_refs:
    - "work/reports/2026-04-22-run-receipt-boundary-research.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Run receipt research is complete. The next honest step is to define a boundary/spec artifact before any `.punk/runs`, receipt schema, or validator implementation.

## Notes

This goal is complete:
- `evals/specs/run-receipt-boundary.v0.1.md` now defines run receipts as future evidence artifacts distinct from event log, eval report, gate decision, and proofpack;
- the spec keeps `.punk/runs`, receipt schema work, validators, and gate/proof implementation deferred;
- the next conservative step is a third advisory Work Ledger Review before selecting the next implementation branch.
