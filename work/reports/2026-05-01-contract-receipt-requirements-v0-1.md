---
id: report_2026_05_01_contract_receipt_requirements_v0_1
kind: work-report
status: final
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
selected_next: work/goals/goal_integrate_contract_gate_input_policy_v0_1.md
---

# Contract Receipt Requirements v0.1

## Summary

Integrated contract receipt requirements as a side-effect-free model/eval seam.

Hard clauses that map to required receipt fields now require matching contract receipt requirements before a ready, explicitly confirmed draft can become `approved_for_run` model state.

No runtime receipt writing, `.punk/runs` storage, CLI, Writer, validator execution, gate writer, or proof writer behavior was added.

## Files changed

- `crates/punk-contract/src/lib.rs` — added receipt requirement model types, assessment, known field set, and approval-boundary integration.
- `crates/punk-eval/src/lib.rs` — added smoke eval coverage for receipt requirement coverage, blockers, non-blocking soft/advisory mappings, executor-claim boundary, and no runtime/gate/proof/Writer activation.
- `docs/product/CRATE-STATUS.md` — recorded the side-effect-free current implemented subset honestly.
- `docs/product/CONTRACT-SCHEMA.md` — clarified receipt requirements boundary.
- `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md` — marked done with completion notes.
- `work/goals/goal_integrate_contract_gate_input_policy_v0_1.md` — created the next selected bounded goal.
- `work/STATUS.md` — recorded completion and selected the gate input policy goal next.
- `work/reports/2026-05-01-contract-receipt-requirements-v0-1.md` — this report.

## What receipt requirements model was added

`punk-contract` now models:

- `ContractReceiptRequirement`;
- `ReceiptRequirementSource`;
- `ReceiptRequirementStatus`;
- `ReceiptRequirementFinding`;
- `ReceiptRequirementCoverage`;
- `ReceiptRequirementAssessment`;
- `assess_receipt_requirements`.

Known receipt fields are:

```text
contract_id
executor_id
started_at
ended_at
inputs
outputs
artifact_refs
artifact_hashes
validator_results
side_effects
deviation_flags
executor_claims
```

The model is declarative and side-effect-free.

## How hard-clause mappings connect to receipt requirements

A hard clause may map to one or more `required_receipt_fields`.

When confirmation evaluates a ready draft, it now checks:

```text
hard clause has required receipt field
-> contract receipt requirements must include that field
-> only then approved_for_run model state can be produced
```

Hard-clause mapping remains separate from runtime validator execution.

## How missing required receipt fields block approval

If a hard clause requires `artifact_hashes`, `validator_results`, `side_effects`, `deviation_flags`, `executor_claims`, or another known field, but the contract receipt requirements do not declare that field, `confirm_contract_draft` returns a blocked result with `missing_required_receipt_field`.

This is model/preflight blocking only. It does not write receipts.

## How unknown or unsupported receipt fields are handled

Unknown receipt fields produce `unsupported_field` findings and block approval through `unsupported_receipt_field`.

The model does not silently accept unknown receipt fields.

Duplicate known requirements are normalized to one required field while preserving a duplicate finding and source information.

## How executor claims remain unverified

`executor_claims` may be required as a future receipt field.

The model explicitly keeps executor claims as unverified evidence:

```text
executor_claims != proof
executor_claims != validator result
executor_claims != gate outcome
```

## Why this does not write receipts

The new model only computes in-memory requirements, findings, coverage, and approval blockers.

It has no file IO, no `.punk/runs` path handling, no receipt writer, and no runtime storage integration.

## Why this does not create gate outcomes or proofpacks

Receipt requirement assessment returns false for gate/proof side effects and has no dependency on gate or proof writers.

It can only declare what future evidence should contain.

## Why Writer remains downstream

Receipt requirements are attached to contract model state before execution.

Writer remains downstream of:

```text
approved contract
-> run receipt/evidence
-> gate outcome
-> proofpack
```

The model does not select, invoke, authorize, or satisfy Writer behavior.

## Eval coverage added

Added smoke coverage for:

- hard clause requiring `artifact_hashes` covered by receipt requirements;
- hard clause requiring `validator_results` covered by receipt requirements;
- hard clause requiring `deviation_flags` covered by receipt requirements;
- hard clause requiring `side_effects` covered by receipt requirements;
- missing receipt requirement blocks `approved_for_run`;
- unknown receipt field is unsupported/blocking;
- duplicate receipt requirements do not create conflicting coverage;
- soft clause receipt field mapping does not block approval if absent;
- advisory clause receipt field mapping does not block approval if absent;
- `executor_claims` remains unverified evidence, not proof;
- receipt requirements do not create run receipts;
- receipt requirements do not write `.punk/runs`;
- receipt requirements do not create gate outcomes;
- receipt requirements do not create proofpacks;
- receipt requirements do not invoke Writer;
- `ContractStatus` still excludes `accepted`, `rejected`, and `partially_accepted`.

## Checks run

- `python3 scripts/check_research_gate.py` — pass.
- `python3 scripts/check_work_ledger.py` — pass.
- `cargo fmt --check` — pass after formatting `crates/punk-contract/src/lib.rs` and `crates/punk-eval/src/lib.rs`.
- `cargo check -p punk-contract` — pass.
- `cargo check -p punk-eval` — pass.
- `cargo test -p punk-contract` — pass, 38 tests.
- `cargo test -p punk-eval` — pass, 6 tests.
- `cargo check --workspace` — pass.
- `cargo test -p punk-contract -p punk-eval` — pass, 38 `punk-contract` tests and 6 `punk-eval` tests.
- `git diff --check` — pass.
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md docs/product/CONTRACT-SCHEMA.md work/goals/goal_integrate_contract_receipt_requirements_v0_1.md work/goals/goal_integrate_contract_gate_input_policy_v0_1.md work/STATUS.md work/reports/2026-05-01-contract-receipt-requirements-v0-1.md --report work/reports/2026-05-01-contract-receipt-requirements-v0-1.md` — pass with 2 existing duplicate-definition warnings in `docs/product/CRATE-STATUS.md`.

## Drift observed

- Existing working tree contains many uncommitted prior goal/report/doc changes. This task used the local uncommitted tree as source of truth and did not attempt to revert or clean unrelated changes.
- Existing docs-governance warnings in `work/STATUS.md` remain recorded for future cleanup.
- Engram persistent memory tools returned `Transport closed`; memory save/session summary could not be completed during implementation.

## Deferred work

- Runtime receipt writer.
- `.punk/runs` storage.
- CLI receipt/contract commands.
- Runtime validator execution.
- Gate input policy implementation.
- Gate outcome writer.
- Proofpack writer activation beyond already-scoped proof writer work.
- Writer activation.
- Provider adapters, policy engine, PKI/signatures, and remote transparency logs.

## Next recommended selected goal

`work/goals/goal_integrate_contract_gate_input_policy_v0_1.md`

Reason:

After hard clauses are mapped and receipt requirements declare future run receipt fields, the next safe seam is modeling which inputs `gate` will require before it can decide.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Integrate side-effect-free contract receipt requirements and document the no-runtime boundary."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - docs/product/CONTRACT-SCHEMA.md
    - work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
    - work/goals/goal_integrate_contract_gate_input_policy_v0_1.md
    - work/STATUS.md
    - work/reports/2026-05-01-contract-receipt-requirements-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - docs/product/CONTRACT-SCHEMA.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
