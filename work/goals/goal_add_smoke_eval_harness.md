---
id: goal_add_smoke_eval_harness
title: "Add smoke eval harness"
status: blocked
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-21
selected_at: null
started_at: null
completed_at: null
blocked_by:
  - "work/goals/goal_connect_flow_transitions_to_event_log.md"
scope:
  include:
    - "crates/punk-eval/**"
    - "evals/**"
    - "docs/product/EVAL.md"
    - "docs/product/EVAL-PLANE.md"
    - "work/**"
  exclude:
    - ".punk/**"
acceptance:
  - "Smoke eval harness exists."
  - "Initial smoke cases can exercise flow and event expectations."
  - "cargo check --workspace passes."
  - "No final decision writing is introduced."
knowledge_refs:
  - "docs/product/EVAL.md"
  - "docs/product/EVAL-PLANE.md"
  - "docs/product/ROADMAP.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Start the smoke eval harness only after there is enough deterministic flow/event behavior to validate.

## Notes

This goal intentionally stays blocked until flow transitions emit deterministic append-only event evidence.
