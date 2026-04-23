---
id: goal_research_gate_decision_boundary
title: "Research gate decision boundary"
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
report_refs: []
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

Keep this goal narrow:
- research only;
- no Rust code;
- no `.punk/`;
- no gate implementation;
- no validator or schema implementation.
