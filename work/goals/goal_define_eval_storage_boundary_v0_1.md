---
id: goal_define_eval_storage_boundary_v0_1
title: "Define eval storage boundary v0.1"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
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
  - "A design/spec artifact defines what future `.punk/evals` storage would treat as canonical run artifacts versus derived views."
  - "The proposal defines explicit non-goals for baseline, waiver, and report persistence implementation."
  - "The proposal states how future eval artifacts may be referenced by gate/proof without making eval a decision surface."
  - "No Rust code, `.punk/evals`, baseline/waiver files, or storage implementation is introduced."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
  - "knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md"
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
  rationale: "R2 research on eval storage, baseline, and waiver boundary is already complete. The next bounded step should be a design/spec artifact only, not implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md"
  external_research_refs:
    - "work/reports/2026-04-22-eval-storage-baseline-boundary-research.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After JSON output exists and storage/baseline/waiver research is complete, the next honest step is a design/spec boundary for future eval storage.

## Notes

Keep this goal narrow:
- design/spec only;
- no implementation;
- no `.punk/evals`;
- no baseline or waiver behavior yet;
- no validator or export work.
