# User intent to contract draft model v0.1

Date: 2026-04-30
Goal: `work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md`
Status: completed
Research Gate: R1, satisfied by repo-tracked product docs, UX boundary spec, Work Ledger state, and the sixty-third advisory review. No external research used.

## Summary

Added a side-effect-free `punk-contract` user intent-to-contract draft model.

The model represents raw user request summary, bounded intent, include/exclude scope, non-goals, assumptions, unknowns, acceptance criteria, evidence plan, Research Gate classification, side-effect boundaries, downstream closure expectations, advisory context-pack refs, and refusal/defer reasons.

It classifies readiness without executing work:

- `ready_for_user_confirmation`
- `clarification_required`
- `refused_or_deferred`
- `blocked`

Selected next: `work/goals/goal_run_sixty_fourth_work_ledger_review.md`.

## Research Gate

- Classification: R1.
- Reason: implementation uses the repo-tracked UX boundary, product docs, crate status, and work-ledger evidence.
- External research: none.
- Escalation preserved: runtime `.punk/contracts` storage, schema files, CLI behavior, contract writer, context-pack writer, executor brief generation, provider/model/agent adapters, automation, gate decision writer, proofpack Writer behavior, acceptance claim writer, and `punk init` remain separate future goals.

## Implementation notes

- Added `USER_INTENT_CONTRACT_DRAFT_MODEL_SCHEMA_VERSION`.
- Added `UserIntentResearchGateClassification` and `UserIntentResearchGate`.
- Added `UserIntentDownstreamClosure` and `user_intent_contract_draft_boundary()`.
- Added `UserIntentContractDraftModel` with builder-style methods for the v0.1 fields.
- Added `UserIntentContractDraftAssessment`, readiness statuses, and blockers.
- Classification stays side-effect-free and in-memory only.
- Context-pack refs remain advisory; they cannot satisfy acceptance criteria by themselves.
- Boundary flags preserve that raw requests are not contracts, intent is not execution permission, user confirmation is not gate acceptance, executor claims are not proof, and Writer is downstream only.
- Added `punk-contract` unit coverage for ready, clarification-required, refused/deferred, blocked, context-pack-advisory, and downstream-authority-drift cases.
- Added `punk-eval` smoke cases for ready, clarification-required, refused/deferred, missing-evidence/research, and Writer-downstream-only behavior.

## Changed files

- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md`
- `work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md`
- `work/goals/goal_run_sixty_fourth_work_ledger_review.md`
- `work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md`

## Knowledge impact

- Punk now has testable upstream intent-to-contract draft semantics before runtime UX, storage, CLI, gate/proof, or Writer work.
- Future Writer work can rely on clearer upstream separation between request, intent, contract draft, user confirmation, execution, evidence, gate, and proof.
- Proofpack Writer referenced artifact verification remains parked until a later review explicitly selects it.

## Boundary behavior preserved

The model does not add:

- runtime `.punk/contracts` storage
- schema files
- CLI behavior
- contract writer
- context-pack writer
- executor brief generation
- provider/model/agent adapters
- automation
- gate decision writer
- proofpack Writer behavior
- active referenced artifact file verification
- acceptance claim writer
- `punk init`

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Tests and smoke coverage

- `punk-contract` unit coverage checks ready, clarification-required, refused/deferred, blocked, context-pack-advisory, and downstream-authority-drift cases.
- `punk-eval` smoke coverage includes intent-to-contract draft model readiness and boundary cases.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Added side-effect-free current crate behavior and smoke coverage for the user intent-to-contract draft model."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md
    - work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md
    - work/goals/goal_run_sixty_fourth_work_ledger_review.md
    - work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test -p punk-contract -p punk-eval
    - cargo check --workspace
```

## Checks run

- `git status --short --branch`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo fmt --all`
- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test -p punk-contract -p punk-eval`
- `git diff --check`
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md work/goals/goal_run_sixty_fourth_work_ledger_review.md work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md --report work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md`

Result: PASS.

Docs governance returned 0 failures and 2 existing CRATE-STATUS definition-candidate warnings.
