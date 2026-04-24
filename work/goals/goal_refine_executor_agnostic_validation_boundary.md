---
id: goal_refine_executor_agnostic_validation_boundary
title: "Refine executor-agnostic validation boundary"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-24
updated_at: 2026-04-24
selected_at: 2026-04-24
started_at: 2026-04-24
completed_at: 2026-04-24
blocked_by: []
scope:
  include:
    - "knowledge/research/2026-04-24-contract-over-prompt.md"
    - "docs/adr/**"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/**"
    - "work/goals/**"
    - "work/reports/**"
    - "work/STATUS.md"
  exclude:
    - "crates/**"
    - "Cargo.toml"
    - "Cargo.lock"
    - ".github/workflows/**"
    - "site/**"
    - "brand/**"
    - ".punk/**"
acceptance:
  - "Boundary wording is corrected from execution-agnostic to executor-agnostic validation."
  - "Docs distinguish user runtime as validation substrate from executor authority."
  - "Executor claims are explicitly not proof."
  - "Semantic assessors are advisory evidence only and cannot write final acceptance."
  - "No Rust code, contract schema change, `.punk/`, provider adapter, runner, prompt manager, or semantic assessor interface is introduced."
knowledge_refs:
  - "knowledge/research/2026-04-24-contract-over-prompt.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/PROJECT-MEMORY.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-24-executor-agnostic-validation-boundary-refinement.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes:
  - "work/goals/goal_execution_agnostic_contract_boundary.md"
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded correction to the previous R2 architecture/policy diff using repo-tracked research plus the user's clarification. No new external Deep Research is needed."
  research_refs:
    - "knowledge/research/2026-04-24-contract-over-prompt.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/RESEARCH-GATE.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/CRATE-STATUS.md"
  rationale: "The diff refines the active architecture wording and eval-policy boundary without changing runtime behavior."
---

## Context

The previous docs-only diff used the shorthand "Punk does not own execution". The user clarified that this is too broad: Punk does run checks in the user's runtime, but must not trust the user's executor as truth.

Corrected principle:

```text
Punk is executor-agnostic, not validation-agnostic.
Punk runs in the user's environment, but does not trust the user's executor as truth.
Punk does not own the executor.
Punk owns contract, validation protocol, evidence, gate, proof, and memory.
```

## Notes

This goal is complete:

- ADR-0014 renamed and rewritten as `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`;
- eval spec renamed and expanded as `evals/specs/executor-agnostic-validation-boundary.v0.1.md`;
- canonical docs now distinguish executor, user runtime, validators, semantic assessors, gate, proof, and memory;
- schema fields and semantic assessor command interfaces remain deferred;
- no Rust code, `.punk/`, provider, runner, prompt manager, or skill manager was added.

The interrupted `selected_next` remains `work/goals/goal_extract_goalrail_process_shell_pilot.md` after this manual correction.
