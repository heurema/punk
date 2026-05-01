# User intent to contract UX flow boundary v0.1

Date: 2026-04-30
Goal: `work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md`
Status: completed
Research Gate: R1, satisfied by repo-tracked product docs, project-memory docs, crate status, Work Ledger state, and the Writer pause report. No external research used.

## Summary

Defined the docs/spec-only boundary for how Punk should move from a user's raw request to understood intent, clarification/refusal when needed, contract draft, and user confirmation before execution.

The boundary keeps Writer downstream: Writer records proof artifacts later; it does not infer intent, approve contracts, write gate decisions, or claim acceptance.

Selected next: `work/goals/goal_run_sixty_third_work_ledger_review.md`.

## Research Gate

- Classification: R1.
- Reason: the boundary uses repo-tracked product architecture, laws, roadmap, crate status, project memory, dogfooding, Work Ledger state, and the Writer pause report.
- External research: none.
- Escalation preserved: runtime contract storage, schema files, CLI behavior, provider/model/agent adapters, automation, context compiler, gate decision writing, proofpack writing, active Writer changes, or `punk init` require separate bounded goals and stronger research gates where required.

## Boundary notes

The new spec defines the product chain:

```text
user request
  -> intent understanding
  -> clarification or refusal when needed
  -> contract draft
  -> user confirmation
  -> approved bounded work
  -> execution
  -> receipt/evidence
  -> eval/assessment
  -> gate decision
  -> proofpack
  -> acceptance claim
```

Key points:

- a raw user request is not a contract;
- intent is Punk's bounded interpretation, not execution permission;
- user confirmation authorizes bounded execution, not acceptance;
- missing material scope, side effects, validation, or authority require clarification;
- hidden authority, unbounded autonomy, unsafe side effects, or parked capability activation require refusal, deferral, or a separate bounded goal;
- evidence planning must happen before non-trivial execution;
- executor claims are not proof;
- Contract Context Packs may support a contract but cannot become the contract or final decision;
- Writer stays downstream of intent, contract, execution, evidence, gate, and proof.

## Created artifact

- `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md`

## Changed files

- `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md`
- `work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md`
- `work/goals/goal_run_sixty_third_work_ledger_review.md`
- `work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md`

## Knowledge impact

- Punk now has an explicit advisory/design boundary for the UX step before contract drafting.
- The boundary explains why deeper Writer work should not own upstream intent or contract decisions.
- The parked Writer boundary is no longer blocked by the absence of an intent-to-contract boundary, but it should remain parked until the next advisory review selects the safest continuation.
- No canonical runtime behavior changed.

## Scope boundaries preserved

No changes were made to:

- `crates/**`
- `.punk/**`
- `schemas/**`
- runtime storage
- CLI behavior
- provider/model/agent adapters
- automation
- context compiler
- Knowledge Vault implementation
- compiled wiki behavior
- active proofpack Writer behavior
- active referenced artifact file verification
- gate decision writer
- acceptance claim writer
- `punk init`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Defined an advisory/design UX boundary for user request -> intent -> contract before execution."
  touched_surfaces:
    - evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md
    - work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md
    - work/goals/goal_run_sixty_third_work_ledger_review.md
    - work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md
  required_updates:
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git status --short --branch`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `git diff --check`
- `scripts/check.sh docs-governance --files evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md work/STATUS.md work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md work/goals/goal_run_sixty_third_work_ledger_review.md work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md --report work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md`

Result: PASS.

Docs governance returned 0 failures and 0 warnings for the UX boundary artifact set.

No Rust/code/schema/CLI changes were made, so cargo checks were not rerun.
