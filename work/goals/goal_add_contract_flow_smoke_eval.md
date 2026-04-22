---
id: goal_add_contract_flow_smoke_eval
title: "Add contract-flow smoke eval"
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
    - "crates/punk-eval/**"
    - "crates/punk-flow/**"
    - "crates/punk-contract/**"
    - "work/**"
    - "evals/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "knowledge/research/**"
acceptance:
  - "Smoke eval coverage includes contract-aware flow guard behavior for approved-for-run, draft, and invalid-scope contract facts."
  - "The step stays eval-only and does not introduce `.punk/`, CLI, run receipts, gate/proof, or storage behavior."
  - "If new eval helpers are added, they remain deterministic and local-only."
  - "The goal does not expand into contract storage, agent execution, module host, adapters, or baseline/waiver work."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "work/reports/2026-04-22-connect-contract-to-flow-state.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-contract-flow-smoke-eval.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "The new contract-to-flow guard should land with deterministic smoke coverage before any storage, receipt, gate, or proof work continues."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
  external_research_refs:
    - "work/reports/2026-04-22-connect-contract-to-flow-state.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After contract state can influence flow guards, the next bounded step is to add deterministic smoke eval coverage for that integration before expanding runtime lifecycle surfaces.

## Notes

This goal is complete:
- `crates/punk-eval/src/lib.rs` now covers the contract-aware flow guard with deterministic smoke cases for approved, draft, invalid-scope, and non-mutating denial behavior;
- the existing smoke report shape and JSON renderer now include the new contract-flow cases without schema changes;
- the next conservative step is to research the run receipt boundary before any receipt, storage, gate, or proof implementation.
