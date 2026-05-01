---
id: goal_define_user_intent_to_contract_ux_flow_boundary_v0_1
title: "Define user intent to contract UX flow boundary v0.1"
status: done
owner: "vitaly"
module: "product"
priority: P1
authority: canonical
created_at: 2026-04-30
updated_at: 2026-04-30
selected_at: 2026-04-30
started_at: 2026-04-30
completed_at: 2026-04-30
blocked_by: []
scope:
  include:
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOGFOODING.md"
    - "evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A docs/spec-only boundary defines the top-level UX chain from user request to understood intent to contract draft before execution."
  - "The boundary defines what the user provides, what Punk treats as intent, when clarifying questions are required, and when Punk must refuse to proceed."
  - "The boundary defines what minimum acceptance criteria and evidence expectations are captured before execution."
  - "The boundary defines where user confirmation happens and distinguishes intent understanding from contract acceptance, execution, gate acceptance, proofpack writing, and positive acceptance claims."
  - "The boundary places execution, receipts/evidence, gate, proofpack, and proofpack Writer as downstream stages without implementing them."
  - "The proofpack Writer track remains parked until this upstream UX boundary is complete."
  - "No runtime/code/schema/CLI behavior, `.punk` writes, active Writer changes, provider/model/agent adapters, automation, context compiler, Knowledge Vault implementation, compiled wiki behavior, gate decision writer, acceptance claim writer, or `punk init` behavior is added."
  - "Exactly one next ready goal is selected after this boundary work."
knowledge_refs:
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/DOGFOODING.md"
  - "work/STATUS.md"
  - "work/reports/2026-04-30-proofpack-writer-track-pause-for-user-intent-alignment-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The boundary should use repo-tracked product docs, project memory, crate status, and current work-ledger evidence to define Punk's upstream UX flow; no external research is needed unless the work expands into broader UX research or implementation."
  research_refs:
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOGFOODING.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-proofpack-writer-track-pause-for-user-intent-alignment-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"
    - "work/reports/**"
    - "work/STATUS.md"
  rationale: "The next step should define a UX boundary only, without changing crate/runtime behavior."
---

## Context

The proofpack Writer is downstream: it records proof artifacts after Punk has understood intent, formed a contract, executed work, and collected evidence.

The current product gap is upstream. Punk needs an explicit UX boundary for how a user's request becomes an understood intent and then a contract draft before execution starts.

## Notes

Do not implement runtime behavior in this goal.
Do not change `crates/**`, `.punk/**`, `schemas/**`, CLI behavior, provider/model/agent adapters, automation, context compiler, Knowledge Vault implementation, compiled wiki behavior, active Writer behavior, gate decision writing, acceptance claim writing, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.
## Outcome

Completed the docs/spec-only user intent-to-contract UX flow boundary v0.1.

Created `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md`.

Selected next: `work/goals/goal_run_sixty_third_work_ledger_review.md`.

No runtime/code/schema/CLI behavior, `.punk` writes, provider/model/agent adapters, automation, context compiler, Knowledge Vault implementation, compiled wiki behavior, active Writer behavior, gate decision writing, acceptance claim writing, or `punk init` behavior was added.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.
