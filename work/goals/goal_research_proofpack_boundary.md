---
id: goal_research_proofpack_boundary
title: "Research proofpack boundary"
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
  - "A research note defines the future proofpack boundary before implementation."
  - "The recommendation explains how proofpack differs from gate decision, run receipt, eval report, and event evidence."
  - "The note clarifies what proofpack must reference and what remains deferred."
  - "No code, `.punk/`, proofpack implementation, or validator implementation is introduced."
knowledge_refs:
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
  - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
  - "evals/specs/gate-decision-boundary.v0.1.md"
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
  rationale: "Proofpack touches provenance semantics, hash/ref expectations, gate linkage, and storage boundaries, so the next honest step is research-first before any proofpack implementation or spec promotion."
  research_refs:
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
    - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
    - "evals/specs/gate-decision-boundary.v0.1.md"
  external_research_refs:
    - "work/reports/2026-04-22-gate-decision-boundary-v0-1.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Gate decision boundary work is complete. The next trust-surface question is how proofpack should stay distinct from decision authority while still linking receipts, evals, events, and later decisions.

## Notes

Keep this goal narrow:
- research only;
- no Rust code;
- no `.punk/`;
- no proofpack implementation;
- no validators or schema implementation.
