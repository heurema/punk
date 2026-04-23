---
id: goal_research_gate_decision_boundary
title: "Research gate decision boundary"
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
    - "knowledge/research/**"
    - "work/**"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "evals/specs/**"
acceptance:
  - "A research note defines the boundary for future gate decisions before implementation."
  - "The recommendation explains what gate writes, what it may reference, and what must remain assessment-only."
  - "The note distinguishes gate decision from receipt, eval report, event log, and proofpack."
  - "No code, `.punk/`, gate implementation, or validator implementation is introduced."
knowledge_refs:
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
  - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
  - "work/reports/2026-04-22-run-receipt-smoke-eval.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-gate-decision-boundary-research.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R2
  required: true
  rationale: "Receipt-aware contract and smoke surfaces now exist. Before gate implementation, the next honest step is research-first on the final decision boundary so receipt, eval, event, and proof references cannot collapse into implicit decision semantics."
  research_refs:
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
    - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
  external_research_refs:
    - "work/reports/2026-04-22-run-receipt-smoke-eval.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Contract, flow, receipt, and smoke coverage now exist as bounded evidence surfaces. The next trust-surface question is what only `gate` may write, and how gate decisions should reference receipt, eval, event, and proof artifacts later.

## Notes

This goal is complete:
- `knowledge/research/2026-04-22-gate-decision-boundary.md` now defines the advisory boundary for future gate decisions;
- the recommendation keeps `gate` as the only final closure authority and keeps prior surfaces evidence-only;
- the next honest step is a design/spec-only gate decision boundary artifact before any gate or proof implementation.
