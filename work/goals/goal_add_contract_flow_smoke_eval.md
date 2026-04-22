---
id: goal_add_contract_flow_smoke_eval
title: "Add contract-flow smoke eval"
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
report_refs: []
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

Keep this goal narrow:
- smoke eval only;
- no CLI;
- no `.punk/`;
- no run receipts, gate, or proof;
- no contract storage or broader flow redesign.
