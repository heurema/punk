---
id: goal_research_eval_storage_and_baseline_boundary
title: "Research eval storage and baseline boundary"
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
    - "work/**"
    - "docs/product/EVAL-PLANE.md"
    - "docs/product/RESEARCH-GATE.md"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "A research note compares bounded options for eval storage, report history, baseline comparison, and waiver semantics before implementation."
  - "The note distinguishes what belongs in repo-tracked truth versus future runtime `.punk/evals` state."
  - "The recommendation classifies follow-ups as adopt, defer, park, or avoid."
  - "No code, `.punk/evals`, or storage implementation is introduced."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/EVAL-PLANE.md"
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
  rationale: "Eval storage, baseline, waiver, and report history all touch storage model and eval policy boundaries, so the next honest step is research-first."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After opt-in JSON output exists, the next tempting steps are `.punk/evals` storage, report history, baseline comparison, and waiver behavior.

## Notes

Keep this goal narrow:
- research only;
- no implementation;
- no `.punk/evals`;
- no baseline or waiver behavior yet;
- no validator or stable-public-contract hardening.
