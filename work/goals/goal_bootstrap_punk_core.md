---
id: goal_bootstrap_punk_core
title: "Bootstrap punk core-first workspace"
status: done
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-21
selected_at: 2026-04-21
started_at: 2026-04-21
completed_at: 2026-04-21
blocked_by: []
scope:
  include:
    - "Cargo.toml"
    - "Cargo.lock"
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
report_refs:
  - "work/reports/2026-04-21-phase-0-workspace-skeleton.md"
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

Phase 0 is complete when the compile-only active-core workspace skeleton exists and `cargo check --workspace` passes without promoting parked behavior.
