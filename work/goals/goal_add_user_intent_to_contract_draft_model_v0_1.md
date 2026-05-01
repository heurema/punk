---
id: goal_add_user_intent_to_contract_draft_model_v0_1
title: "Add user intent to contract draft model v0.1"
status: done
owner: "vitaly"
module: "contract"
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
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A side-effect-free `punk-contract` model represents raw user request summary, bounded intent, scope include/exclude, non-goals, assumptions, unknowns, acceptance criteria, evidence plan, Research Gate classification, side-effect boundaries, and downstream closure expectations."
  - "The model classifies contract draft readiness without executing work: ready for user confirmation, clarification required, refused/deferred, or blocked by missing evidence/research/scope."
  - "The model preserves that a raw user request is not a contract, intent is not execution permission, user confirmation is not gate acceptance, and executor claims are not proof."
  - "The model keeps Contract Context Pack refs advisory and prevents them from defining acceptance criteria or final decisions."
  - "`punk-eval` has smoke coverage for ready, clarification-required, refused/deferred, missing-evidence, and Writer-downstream-only cases."
  - "CRATE-STATUS is updated to describe the new side-effect-free current behavior accurately."
  - "No runtime `.punk/contracts` storage, schema files, CLI behavior, contract writer, context-pack writer, executor brief generation, provider/model/agent adapters, automation, gate decision writer, proofpack writer behavior, acceptance claim writer, or `punk init` behavior is added."
  - "Proofpack Writer referenced artifact verification remains parked during this goal."
knowledge_refs:
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/DOGFOODING.md"
  - "evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"
  - "work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md"
  - "work/reports/2026-04-30-sixty-third-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The implementation should use the repo-tracked intent-to-contract UX boundary and canonical product docs to add a side-effect-free in-memory model only; external research is not needed unless runtime UX, schema, CLI, storage, adapters, or automation are added."
  research_refs:
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOGFOODING.md"
    - "evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md"
    - "work/reports/2026-04-30-sixty-third-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The selected next step should add side-effect-free crate behavior and update current-behavior documentation."
---

## Context

The user intent-to-contract UX flow boundary v0.1 is now defined as an advisory/design spec.

The next safe implementation step is not runtime UX, CLI, storage, provider execution, gate/proof, or proofpack Writer work. It is a side-effect-free in-memory model that makes the boundary testable before any active runtime surface exists.

## Notes

Do not add runtime storage, schema files, CLI behavior, `.punk` writes, provider/model/agent adapters, automation, context compiler behavior, gate decision writing, proofpack Writer behavior, acceptance claim writing, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.
## Outcome

Completed the side-effect-free user intent-to-contract draft model v0.1.

Added in-memory model behavior in `crates/punk-contract/src/lib.rs`, smoke coverage in `crates/punk-eval/src/lib.rs`, and current-behavior documentation in `docs/product/CRATE-STATUS.md`.

Selected next: `work/goals/goal_run_sixty_fourth_work_ledger_review.md`.

No runtime `.punk/contracts` storage, schema files, CLI behavior, contract writer, context-pack writer, executor brief generation, provider/model/agent adapters, automation, gate decision writer, proofpack Writer behavior, acceptance claim writer, or `punk init` behavior was added.

Proofpack Writer referenced artifact verification remains parked.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.
