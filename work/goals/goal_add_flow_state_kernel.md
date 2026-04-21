---
id: goal_add_flow_state_kernel
title: "Add flow state kernel"
status: ready
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-21
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-flow/**"
    - "docs/product/FLOW.md"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-events/**"
    - "crates/punk-eval/**"
acceptance:
  - "Flow state types exist."
  - "Transition guard is deterministic and pure."
  - "Illegal transition is denied in unit test."
  - "Allowed transition returns the next state in unit test."
  - "cargo check --workspace passes."
  - "No .punk writes occur."
  - "No CLI runtime claims are introduced."
knowledge_refs:
  - "docs/product/FLOW.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/START-HERE.md"
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

Build the smallest useful pure flow-state kernel before persistence, event writing, CLI inspect behavior, or eval harness work.

## Notes

This is the next implementation-ready goal after the Phase 0 workspace skeleton.
