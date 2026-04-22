---
id: goal_add_smoke_eval_json_output_v0_1
title: "Add smoke eval JSON output v0.1"
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
    - "crates/punk-eval/**"
    - "crates/punk-cli/**"
    - "evals/specs/smoke-eval-report.v0.1.md"
    - "work/**"
  exclude:
    - ".punk/**"
    - "evals/_schema/**"
    - "docs/product/**"
acceptance:
  - "`punk-eval` can map the existing internal smoke report to the proposed `smoke-eval-report.v0.1` JSON shape."
  - "`punk eval run smoke` can emit the bounded v0.1 JSON output without implying `.punk/evals` storage."
  - "No baseline, waiver, report persistence, validator engine, or stable public contract hardening is introduced."
  - "Human-readable output remains available and assessment-only semantics are preserved."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
  - "knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md"
  - "evals/specs/smoke-eval-report.v0.1.md"
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
  rationale: "Implements a narrow JSON output over an already researched schema proposal. Escalate before editing if the diff expands into storage, baseline/waiver, validator, or stable-public-contract work."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md"
    - "evals/specs/smoke-eval-report.v0.1.md"
  external_research_refs:
    - "work/reports/2026-04-22-eval-report-schema-research.md"
    - "work/reports/2026-04-22-smoke-eval-report-schema-v0-1.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the schema-only proposal exists, expose one bounded machine-readable output path without activating storage or wider eval-platform behavior.

## Notes

Keep this goal narrow:
- JSON output only;
- no `.punk/evals`;
- no baseline or waiver behavior;
- no schema validator or report persistence;
- no decision, gate, or proof semantics.
