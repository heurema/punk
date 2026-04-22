---
id: goal_add_contract_lifecycle_minimal
title: "Add minimal contract lifecycle"
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
    - "crates/**"
    - "work/**"
    - "evals/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "knowledge/research/**"
acceptance:
  - "A narrow contract lifecycle surface exists for a bounded contract with explicit scope."
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
report_refs:
  - "work/reports/2026-04-22-contract-lifecycle-minimal.md"
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

This goal is complete:
- `crates/punk-contract/src/lib.rs` now defines minimal contract primitives, scope validation, and bounded approval-for-run semantics;
- the kernel stays pure and does not introduce `.punk/`, CLI, gate/proof, or run execution;
- the next bounded step is to connect the new contract kernel to existing flow state without activating runtime storage.
