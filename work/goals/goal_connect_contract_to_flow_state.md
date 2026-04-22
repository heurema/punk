---
id: goal_connect_contract_to_flow_state
title: "Connect contract to flow state"
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
    - "crates/punk-contract/**"
    - "crates/punk-flow/**"
    - "work/**"
    - "evals/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "knowledge/research/**"
acceptance:
  - "Existing flow state can reference minimal contract scope/status semantics without introducing runtime storage."
  - "The connection stays kernel-only and does not add CLI behavior, `.punk/`, gate/proof, or run receipt implementation."
  - "If contract-aware flow validation is added, it is covered by tests."
  - "The goal does not expand into contract storage, agent execution, module host, adapters, or eval storage."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
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
  rationale: "The minimal contract kernel now exists, and the next bounded step is to connect it to the existing flow kernel without expanding into storage, CLI, gate, or proof behavior."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
  external_research_refs:
    - "work/reports/2026-04-22-contract-lifecycle-minimal.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the minimal contract kernel exists, the next bounded step is to connect contract scope/status semantics to the existing flow kernel.

## Notes

Keep this goal narrow:
- kernel-only integration;
- no CLI;
- no `.punk/`;
- no gate/proof or run receipt behavior;
- no module host, adapters, or full contract storage.
