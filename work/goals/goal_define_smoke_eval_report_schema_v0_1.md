---
id: goal_define_smoke_eval_report_schema_v0_1
title: "Define smoke eval report schema v0.1"
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
    - "evals/_schema/**"
    - "docs/product/EVAL-PLANE.md"
    - "work/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "A schema-only proposal defines the bounded machine-readable smoke eval report fields and versioning policy for v0.1."
  - "The proposal distinguishes canonical internal report shape from future export contract."
  - "No `.punk/evals` storage, baseline, waiver, or CLI/runtime implementation is introduced."
  - "Required evals and doc/ADR implications are listed."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
  - "docs/product/EVAL.md"
  - "knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md"
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
  rationale: "Defines a stable machine-readable smoke eval report contract boundary, which touches eval report schema and future public CLI/storage implications."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "docs/product/EVAL.md"
    - "knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md"
  external_research_refs:
    - "work/reports/2026-04-22-eval-report-schema-research.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After research clarified the boundary around machine-readable output, define a schema-only proposal before any JSON output, runtime storage, baseline, or waiver work.

## Notes

Keep this goal narrow:
- schema/design only
- no code changes
- no `.punk/evals`
- no stable export implementation yet
- no baseline or waiver behavior
