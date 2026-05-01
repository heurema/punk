# Contract draft confirmation boundary v0.1

Date: 2026-04-30
Goal: `work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md`
Status: completed
Research Gate: R1, satisfied by repo-tracked product docs, Contract Schema Blueprint, intent-to-contract draft model evidence, and the sixty-fourth advisory review. No external research used.

## 1. Summary

Implemented a side-effect-free contract draft confirmation boundary.

The new boundary models this upstream seam:

```text
ready contract draft
-> explicit user confirmation
-> approved_for_run model state
```

It does not activate runtime behavior, CLI, `.punk/contracts` storage, Writer, gate writer, proof writer, agents, adapters, policy engine, signatures, or remote transparency.

Selected next:

```text
work/goals/goal_integrate_hard_clause_mapping_v0_1.md
```

## 2. Files changed

- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md`
- `work/goals/goal_integrate_hard_clause_mapping_v0_1.md`
- `work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md`

## 3. Model added/extended

Added in `punk-contract`:

- `CONTRACT_DRAFT_CONFIRMATION_BOUNDARY_SCHEMA_VERSION`
- `ContractDraftConfirmationBoundary`
- `ContractDraftApprovalEvidence`
- `ContractDraftConfirmation`
- `ContractDraftUnknownDisposition`
- `ContractDraftUnknownHandling`
- `ContractDraftConfirmationOutcome`
- `ContractDraftConfirmationBlocker`
- `ApprovedForRunContractDraft`
- `ContractDraftConfirmationResult`
- `confirm_contract_draft(...)`

The approved-for-run model preserves structured upstream content instead of flattening it into prose.

## 4. Confirmation vs gate acceptance

User confirmation now means:

```text
The user explicitly confirms that this ready draft is the bounded contract to approve for run.
```

It does not mean:

- work is accepted;
- gate passed;
- proof exists;
- Writer should run;
- runtime storage should be written.

The approved output exposes false boundary methods for gate/proof/Writer authority:

- `creates_gate_acceptance() == false`
- `writes_gate_decision() == false`
- `creates_proofpack() == false`
- `creates_acceptance_claim() == false`
- `invokes_writer() == false`

## 5. Preventing blocked/refused/clarification approval

`confirm_contract_draft(...)` first checks the draft readiness classification.

Only `ready_for_user_confirmation` can proceed toward approval.

These outcomes cannot approve:

- `clarification_required`
- `refused_or_deferred`
- `blocked`

They return corresponding confirmation outcomes and no approved-for-run output.

## 6. Unknown handling

Ready drafts with unresolved unknowns cannot approve silently.

Each unknown must be handled explicitly as one of:

- `resolved`
- `converted_to_assumption`
- `explicitly_accepted_risk`

Converted assumptions are carried into the approved-for-run model assumptions.

Resolved unknowns and risk-accepted unknowns remain visible as structured lists.

## 7. Writer downstream boundary

Writer remains downstream.

The confirmation boundary does not invoke, select, authorize, or depend on Writer.

The intended order remains:

```text
approved contract
-> run receipt/evidence
-> gate decision
-> proofpack
-> Writer
```

## 8. Eval coverage added

Added `punk-eval` smoke coverage for:

- ready draft with explicit confirmation becomes `approved_for_run`;
- ready draft without explicit confirmation is not approved;
- clarification-required draft cannot be approved;
- refused/deferred draft cannot be approved;
- blocked draft cannot be approved;
- unresolved unknowns block approval;
- unknowns converted to assumptions allow approval;
- approved-for-run state preserves scope and non-scope;
- approved-for-run state preserves evidence plan;
- approved-for-run state preserves side-effect boundaries;
- user confirmation does not create gate decision;
- user confirmation does not create proofpack;
- user confirmation does not invoke Writer;
- `ContractStatus` still excludes `accepted`, `rejected`, and `partially_accepted`.

Added `punk-contract` unit coverage for the same core boundary behavior.

## 9. Checks run

- `git status --short --branch`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo fmt --all`
- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test -p punk-contract -p punk-eval`
- `git diff --check`
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md work/goals/goal_integrate_hard_clause_mapping_v0_1.md work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md --report work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md`

Result: PASS.

Docs governance returned 0 failures and 2 scoped CRATE-STATUS warnings: `Current implemented subset boundary` and `Current CLI surface`.

## 10. Drift observed

No blocking drift found.

Known broader docs-governance warnings remain accepted and tracked in `work/STATUS.md`:

- `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary`
- `docs/product/CRATE-STATUS.md`: `Current CLI surface`
- `docs/product/DOCUMENTATION-MAP.md`: `Research notes`

## 11. Deferred work

Deferred next steps remain:

1. hard-clause mapping integration;
2. receipt requirements integration;
3. gate input policy integration;
4. proof requirements integration;
5. Writer only after upstream/gate/proof seams are ready and a later review selects it.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Added side-effect-free contract draft confirmation model/eval behavior and updated current behavior/work-ledger evidence."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md
    - work/goals/goal_integrate_hard_clause_mapping_v0_1.md
    - work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md
    - work/goals/goal_integrate_hard_clause_mapping_v0_1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test -p punk-contract -p punk-eval
    - cargo check --workspace
```
