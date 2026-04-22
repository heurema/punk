---
id: goal_define_smoke_eval_report_schema_v0_1
title: "Define smoke eval report schema v0.1"
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
    - "evals/specs/smoke-eval-report.v0.1.md"
    - "work/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "A schema-only proposal defines the bounded machine-readable smoke eval report fields and versioning policy for v0.1."
  - "The proposal distinguishes canonical internal report shape from future export contract."
  - "No `.punk/evals` storage, baseline, waiver, or CLI/runtime implementation is introduced."
  - "Required follow-up implications are listed without implementing machine output."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
  - "knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-smoke-eval-report-schema-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "R2 research is already complete. This diff only defines a bounded schema/design proposal and does not implement machine-readable output, storage, validators, baseline/waiver semantics, or eval policy changes."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
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

After research clarified the machine-readable output boundary, define a schema-only proposal before any JSON output, runtime storage, baseline, or waiver work.

## Notes

This goal is complete:
- `evals/specs/smoke-eval-report.v0.1.md` now defines a proposed v0.1 artifact shape;
- the proposal stays advisory/design and does not become a stable public contract yet;
- the next bounded step, if implementation proceeds, is a narrow JSON-output goal over the proposed schema only.
