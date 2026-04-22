---
id: goal_define_eval_baseline_waiver_boundary_v0_1
title: "Define eval baseline and waiver boundary v0.1"
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
  - "A design/spec artifact defines the future baseline and waiver boundary without implementing baseline comparison or waiver files."
  - "The proposal states minimum baseline reference rules and minimum waiver requirements."
  - "The proposal keeps eval as assessment-only and does not weaken Punk laws."
  - "No Rust code, `.punk/evals`, baseline/waiver implementation, or validators are introduced."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
  - "knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md"
  - "evals/specs/eval-storage-boundary.v0.1.md"
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
  rationale: "R2 research and the storage-boundary design are complete. The next bounded step is a baseline/waiver boundary spec only, not implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md"
    - "evals/specs/eval-storage-boundary.v0.1.md"
  external_research_refs:
    - "work/reports/2026-04-22-eval-storage-baseline-boundary-research.md"
    - "work/reports/2026-04-22-eval-storage-boundary-v0-1.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the storage boundary is explicit, the next honest design step is to define the baseline and waiver boundary before any implementation.

## Notes

Keep this goal narrow:
- design/spec only;
- no implementation;
- no `.punk/evals`;
- no baseline or waiver files yet;
- no validators or gate/proof work.
