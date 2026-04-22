---
id: goal_research_eval_report_schema_before_machine_output
title: "Research eval report schema before machine output"
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
    - "knowledge/research/**"
    - "docs/product/EVAL-PLANE.md"
    - "docs/adr/**"
    - "work/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "A research note compares internal-only report rendering, machine-readable output options, and storage-free versus runtime-stored report shapes."
  - "The note extracts failure modes for promoting smoke eval output into a stable schema or storage contract."
  - "The note recommends adopt/defer/park/avoid for machine-readable output, storage, baseline, and waiver follow-ups."
  - "The note lists required ADRs, docs, and evals before any stable report contract or storage work."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
  - "docs/product/EVAL.md"
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
  rationale: "Any stable machine-readable eval output or report contract touches eval storage, report schema, and public CLI contract boundaries."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "docs/product/EVAL.md"
  external_research_refs:
    - "work/reports/2026-04-22-smoke-eval-harness.md"
    - "work/reports/2026-04-22-smoke-eval-cli-command.md"
    - "work/reports/2026-04-22-smoke-eval-report-artifact-shape.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Before adding machine-readable smoke eval output, runtime report storage, or a stable report contract, run a focused research gate over report-shape options and failure modes.

## Notes

Keep this goal narrow:
- research only
- no `.punk/evals` runtime storage
- no stable JSON/schema implementation
- no baseline or waiver behavior
- no gate/proof integration
