---
id: goal_add_contract_lifecycle_minimal
title: "Add minimal contract lifecycle"
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
    - "crates/**"
    - "work/**"
    - "evals/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "knowledge/research/**"
acceptance:
  - "A narrow contract lifecycle surface exists for an accepted contract with explicit scope."
  - "The step stays minimal and does not introduce agents, autonomous cut, gate, proof, module host, adapters, or `.punk/` runtime state."
  - "If code changes are introduced later, the goal adds tests or assertions around the new lifecycle surface."
  - "The goal does not expand into full contract memory, baseline/waiver implementation, or eval storage implementation."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
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
  rationale: "The review recommends returning to Phase 3 core loop work with a minimal bounded contract-lifecycle step. This should remain a narrow implementation over existing roadmap and architecture truth."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
  external_research_refs:
    - "work/reports/2026-04-22-second-work-ledger-review.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the eval/report/storage/baseline boundary sequence, the next bounded step is to return to the Phase 3 core lifecycle with the smallest honest contract artifact or type surface.

## Notes

Keep this goal narrow:
- minimal contract lifecycle only;
- no agent execution;
- no `.punk/` runtime state;
- no gate/proof implementation unless explicitly re-scoped later;
- no module host, adapters, or full project memory behavior.
