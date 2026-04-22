---
id: goal_add_flow_inspect_command
title: "Add flow inspect command"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-22
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-cli/**"
    - "crates/punk-flow/**"
    - "crates/punk-events/**"
    - "docs/product/FLOW.md"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-eval/**"
acceptance:
  - "A bounded `punk flow inspect` surface exists."
  - "The command does not expose parked capability surfaces."
  - "The command makes no dishonest runtime claims."
  - "cargo check --workspace passes."
knowledge_refs:
  - "docs/product/FLOW.md"
  - "docs/product/START-HERE.md"
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

Expose inspectable flow state only after there is real state and event evidence to inspect.

## Notes

Flow transitions now emit event drafts, so this is the next honest inspect surface.
