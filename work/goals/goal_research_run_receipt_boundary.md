---
id: goal_research_run_receipt_boundary
title: "Research run receipt boundary"
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
    - "knowledge/research/**"
    - "work/**"
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "A research note defines the boundary for future run receipts before implementation."
  - "The recommendation distinguishes receipt evidence from gate/proof authority."
  - "The note explains receipt storage, hash/reference, and runtime boundary implications before any `.punk/` activation."
  - "No code, `.punk/`, or run receipt implementation is introduced."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "work/reports/2026-04-22-contract-flow-smoke-eval.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-run-receipt-boundary-research.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R2
  required: true
  rationale: "Run receipts touch runtime storage, lifecycle evidence order, and later gate/proof references, so the next honest step is research-first."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
  external_research_refs:
    - "work/reports/2026-04-22-contract-flow-smoke-eval.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

The contract-to-flow path is now covered by deterministic smoke evals. The next tempting step is run receipts, but receipts sit on the runtime-storage and later gate/proof boundary.

## Notes

This goal is complete:
- `knowledge/research/2026-04-22-run-receipt-boundary.md` now defines the advisory boundary for future run receipts;
- the recommendation keeps receipts as future evidence artifacts, not decision artifacts;
- the next honest step is a design/spec-only run receipt boundary artifact before any `.punk/runs` implementation.
