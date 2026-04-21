---
id: goal_add_flow_and_smoke_eval
title: "Add flow controller and smoke eval foundation"
status: superseded
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-21
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-flow/**"
    - "crates/punk-eval/**"
    - "flows/**"
    - "evals/**"
    - "docs/product/FLOW.md"
    - "docs/product/EVAL.md"
  exclude:
    - "crates/punk-mod-dev/**"
    - "crates/punk-mod-pub/**"
    - "crates/punk-adapters/**"
acceptance:
  - "Illegal transitions are denied."
  - "Next allowed command is inspectable."
  - "Smoke eval suite can run."
  - "Every transition writes an event."
knowledge_refs:
  - "docs/product/FLOW.md"
  - "docs/product/EVAL.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-21-split-flow-smoke-eval-goal.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by:
  - "work/goals/goal_add_flow_state_kernel.md"
  - "work/goals/goal_add_append_only_event_log.md"
  - "work/goals/goal_add_flow_inspect_command.md"
  - "work/goals/goal_add_smoke_eval_harness.md"
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Build Phase 1 and Phase 2 foundation after Phase 0 skeleton is stable.

## Notes

This broad goal was superseded because it mixed multiple roadmap phases and exceeded the intended bounded-work size for the next implementation step.
