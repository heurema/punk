---
id: report_2026_05_01_contract_gate_input_policy_v0_1
kind: work-report
status: final
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_integrate_contract_gate_input_policy_v0_1.md
selected_next: work/goals/goal_integrate_contract_proof_requirements_v0_1.md
---

# Contract Gate Input Policy v0.1

## Summary

Integrated contract gate input policy as a side-effect-free model/eval seam.

The model declares what a future gate must inspect before it can be ready to write an outcome. It does not write gate outcomes, create proofpacks, create acceptance claims, invoke Writer, run validators, or write storage.

The key lifecycle boundary is preserved:

```text
approved_for_run != ready_for_gate
gate outcome -> proofpack -> acceptance claim
```

Proofpack is not a gate input.

## Files changed

- `crates/punk-contract/src/lib.rs` — added gate input policy/readiness model types, assessment, known input set, no-proofpack-input boundary, and approved-for-run/gate-readiness separation.
- `crates/punk-eval/src/lib.rs` — added smoke eval coverage for required gate inputs, missing/unsupported inputs, duplicate normalization, no proofpack-as-input behavior, no gate/proof/acceptance/Writer/runtime activation, and unchanged `ContractStatus` boundaries.
- `docs/product/CRATE-STATUS.md` — recorded the side-effect-free current implemented subset honestly.
- `docs/product/CONTRACT-SCHEMA.md` — clarified gate input policy boundary and proofpack ordering.
- `work/goals/goal_integrate_contract_gate_input_policy_v0_1.md` — marked done with completion notes.
- `work/goals/goal_integrate_contract_proof_requirements_v0_1.md` — created the next selected bounded goal.
- `work/STATUS.md` — recorded completion and selected proof requirements next.
- `work/reports/2026-05-01-contract-gate-input-policy-v0-1.md` — this report.

## What gate input policy model was added

`punk-contract` now models:

- `ContractGateInputPolicy`;
- `GateInputRequirement`;
- `GateInputRequirementSource`;
- `GateInputEvidence`;
- `GateInputRequirementStatus`;
- `GateInputPolicyFinding`;
- `GateInputCoverage`;
- `GateInputPolicyAssessment`;
- `assess_gate_input_policy`.

Known gate inputs are:

```text
contract_ref
approved_for_run_status
run_receipt_ref
receipt_requirement_coverage
hard_clause_mapping_assessment
validator_or_eval_report
module_or_semantic_assessment
scope_status
deviation_status
executor_claim_status
artifact_refs
artifact_hashes
```

Baseline required inputs are:

```text
contract_ref
approved_for_run_status
run_receipt_ref
receipt_requirement_coverage
hard_clause_mapping_assessment
validator_or_eval_report
scope_status
deviation_status
executor_claim_status
```

## How gate inputs relate to contract, hard clauses, receipt requirements, evals, deviations, and executor claims

Gate input policy declares that future gate readiness depends on:

- contract identity;
- evidence that the contract reached `approved_for_run`;
- run receipt evidence;
- receipt requirement coverage;
- hard-clause mapping assessment;
- validator/eval/assessment refs when applicable;
- scope status;
- deviation status;
- executor claim status.

This keeps gate readiness downstream of run evidence and distinct from contract approval.

Executor claim status is visible to gate as evidence classification only. Executor claims do not become proof.

## Why gate input policy does not write gate decisions

The assessment only computes required inputs, findings, and coverage.

It returns no gate outcome and has explicit false boundary methods for gate-writing behavior.

Only future `gate` may write the final outcome.

## Why gate input policy does not require proofpack

Proofpack is downstream of gate outcome:

```text
gate outcome -> proofpack -> acceptance claim
```

The policy rejects proofpack-shaped inputs such as `proofpack`, `proofpack_ref`, `existing_proofpack`, and `proofpack_writer_output` as unsupported gate inputs.

The model may expose `post_gate_proof_requirement_pending`, but that is a downstream expectation, not gate input coverage.

## Why approved_for_run is not ready_for_gate

`approved_for_run` means a contract draft was explicitly confirmed and is ready for bounded execution.

`ready_for_gate` means post-run evidence exists and covers the gate input policy.

A contract can be approved for run before any run receipt exists. Therefore `approved_for_run` alone is not gate readiness.

## Why this does not create proofpacks or acceptance claims

Gate input policy assessment has no proofpack writer, no acceptance claim writer, and no storage behavior.

It can identify future post-gate proof expectations but cannot create proof artifacts or acceptance authority.

## Why Writer remains downstream

Writer remains downstream of:

```text
approved contract
-> run receipt/evidence
-> gate outcome
-> proofpack
```

The model does not select, invoke, authorize, or depend on Writer.

## Eval coverage added

Added smoke coverage for:

- gate input policy requires contract ref;
- gate input policy requires `approved_for_run` status;
- gate input policy requires run receipt ref;
- gate input policy requires receipt requirement coverage;
- gate input policy requires hard-clause mapping assessment;
- gate input policy can require validator/eval report refs;
- gate input policy can require deviation status;
- gate input policy can require executor claim status without treating claims as proof;
- missing contract ref blocks gate readiness;
- missing `approved_for_run` status blocks gate readiness;
- missing run receipt ref blocks gate readiness;
- missing receipt requirement coverage blocks gate readiness;
- missing hard-clause mapping assessment blocks gate readiness;
- unsupported gate input blocks gate readiness;
- duplicate gate input requirements do not create conflicting coverage;
- gate input policy does not write gate outcomes;
- gate input policy does not create proofpacks;
- gate input policy does not require existing proofpack;
- gate input policy does not create acceptance claims;
- gate input policy does not invoke Writer;
- `ContractStatus` still excludes `accepted`, `rejected`, and `partially_accepted`;
- `approved_for_run` is not `ready_for_gate`.

## Checks run

- `python3 scripts/check_research_gate.py` — pass.
- `python3 scripts/check_work_ledger.py` — pass.
- `cargo fmt --check` — pass.
- `cargo check --workspace` — pass.
- `cargo test -p punk-contract -p punk-eval` — pass, 39 `punk-contract` tests and 6 `punk-eval` tests.
- `git diff --check` — pass.
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md docs/product/CONTRACT-SCHEMA.md work/goals/goal_integrate_contract_gate_input_policy_v0_1.md work/goals/goal_integrate_contract_proof_requirements_v0_1.md work/STATUS.md work/reports/2026-05-01-contract-gate-input-policy-v0-1.md --report work/reports/2026-05-01-contract-gate-input-policy-v0-1.md` — pass with 2 existing duplicate-definition warnings in `docs/product/CRATE-STATUS.md`.

## Drift observed

- Existing working tree contains many uncommitted prior goal/report/doc changes. This task used the local uncommitted tree as source of truth and did not attempt to revert unrelated work.
- Existing docs-governance duplicate-definition warnings in `docs/product/CRATE-STATUS.md` remain expected until a bounded docs-governance cleanup.
- The previous selected goal text mentioned proof requirements as possible gate input context; this task corrected the boundary so proofpack/proofpack output is not a gate prerequisite.
- Engram persistent memory tools returned `Transport closed`; memory save/session summary may fail until the transport is restored.

## Deferred work

- Runtime gate decision writer.
- `.punk/decisions` storage.
- Runtime receipt writer and `.punk/runs` storage.
- Proof requirements integration.
- Runtime proofpack writer and `.punk/proofs` storage.
- Acceptance claim writer.
- Writer activation.
- Runtime validator execution.
- Provider adapters, policy engine, PKI/signatures, and remote transparency logs.

## Next recommended selected goal

`work/goals/goal_integrate_contract_proof_requirements_v0_1.md`

Reason:

After contract clauses, receipt requirements, and gate input policy are modeled, the next safe seam is downstream proof requirements: what proofpack must later link/hash after gate outcome.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Integrate side-effect-free contract gate input policy and preserve proofpack as downstream of gate outcome."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - docs/product/CONTRACT-SCHEMA.md
    - work/goals/goal_integrate_contract_gate_input_policy_v0_1.md
    - work/goals/goal_integrate_contract_proof_requirements_v0_1.md
    - work/STATUS.md
    - work/reports/2026-05-01-contract-gate-input-policy-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - docs/product/CONTRACT-SCHEMA.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
