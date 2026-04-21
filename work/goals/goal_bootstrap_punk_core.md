---
id: goal_bootstrap_punk_core
title: "Bootstrap punk core-first workspace"
status: ready
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
    - "Cargo.toml"
    - "crates/**"
    - "docs/product/**"
    - "docs/adr/**"
    - "flows/**"
    - "evals/**"
    - "work/**"
    - "knowledge/**"
  exclude:
    - ".punk/**"
acceptance:
  - "Workspace skeleton exists."
  - "Core crate statuses are documented."
  - "Parked capability boundaries are documented but not active."
  - "cargo check --workspace passes."
  - "Phase 0 docs exist."
knowledge_refs:
  - "docs/product/START-HERE.md"
  - "docs/product/ROADMAP.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Create the initial `punk` repo skeleton without enabling modules, adapters, LLM execution, or external side effects.

## Notes

This is Dogfooding Level 0: the work is tracked in `punk` project memory before `punk` can execute it.
