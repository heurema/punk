---
id: report_2026_05_01_contract_proof_requirements_v0_1
kind: work-report
status: final
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_integrate_contract_proof_requirements_v0_1.md
selected_next: work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md
---

# Contract Proof Requirements v0.1

## Summary

Integrated contract proof requirements as a side-effect-free declaration/model/eval seam.

The model declares what a future proofpack must link/hash after a gate outcome exists. It does not create proofpacks, compute artifact hashes, write `.punk/proofs`, write gate outcomes, create acceptance claims, invoke Writer, run validators, or write runtime storage.

The lifecycle boundary is preserved:

```text
contract
  -> run receipt/evidence
  -> gate outcome
  -> proofpack
  -> acceptance claim
```

Proof requirements are not proofpack writer behavior.

## Files changed

- `crates/punk-contract/src/lib.rs` — added/extended side-effect-free proof requirements declaration and assessment types, known/baseline proof targets, duplicate normalization, missing/unsupported blockers, default contract proof requirements on drafts/approved-for-run model state, and no-writer/no-storage/no-hash-runtime boundary methods.
- `crates/punk-eval/src/lib.rs` — added smoke eval coverage for proof requirement targets, missing/unsupported/duplicate target handling, proofpack/hash/storage/gate/acceptance/Writer boundaries, no proofpack-before-gate behavior, and unchanged `ContractStatus` acceptance boundaries.
- `docs/product/CRATE-STATUS.md` — recorded the side-effect-free current implemented subset honestly.
- `docs/product/CONTRACT-SCHEMA.md` — clarified proof requirements as future proofpack link/hash declarations after gate outcome.
- `work/goals/goal_integrate_contract_proof_requirements_v0_1.md` — marked done with completion notes.
- `work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md` — created the next review checkpoint goal.
- `work/STATUS.md` — recorded completion and selected the review checkpoint next.
- `work/reports/2026-05-01-contract-proof-requirements-v0-1.md` — this report.

## What proof requirements model was added/extended

`punk-contract` now models:

- `ContractProofRequirements`;
- `ProofRequirement`;
- `ProofRequirementSource`;
- `ProofRequirementStatus`;
- `ProofRequirementFinding`;
- `ProofRequirementCoverage`;
- `ProofRequirementAssessment`;
- `assess_proof_requirements`.

Known proof targets are:

```text
contract_ref
contract_hash
run_receipt_ref
run_receipt_hash
gate_decision_ref
gate_decision_hash
eval_report_ref
eval_report_hash
output_artifact_refs
output_artifact_hashes
event_range_or_root
rule_or_guard_version_refs
```

Baseline required targets are:

```text
contract_ref
run_receipt_ref
gate_decision_ref
```

The default proof requirements also include contract/run/gate hashes plus eval report and output artifact refs/hashes for future proofpack linkage.

## How proof requirements relate to contract, run receipt, gate decision, eval reports, and artifacts

Proof requirements declare the future proofpack target set:

- contract identity and hash;
- run receipt ref and hash;
- gate outcome ref and hash;
- eval report refs/hashes when applicable;
- output artifact refs/hashes when expected.

They are carried on the contract draft and approved-for-run model state as declarations only.

A contract can be `approved_for_run` before gate outcome or proofpack evidence exists. Proof readiness is downstream and cannot collapse into contract approval.

## Why proof requirements do not create proofpacks

The model only returns required targets, coverage, and findings.

It has explicit false boundary methods for proofpack creation and `.punk/proofs` storage. There is no manifest rendering, file writing, index update, latest-pointer update, or persisted operation evidence in this task.

## Why proof requirements do not compute hashes

Proof requirements may declare that future hashes are required.

They do not read files, normalize bytes, compute artifact hashes, compute tree hashes, verify artifact contents, or invoke `punk-core` file hashing helpers. Hash computation remains a separate explicit boundary and is not activated here.

## Why proof requirements do not write gate decisions or acceptance claims

Proof requirements are downstream declarations for proofpack shape. They do not accept or reject work and do not write gate outcomes.

Acceptance claims remain downstream and require an accepting gate outcome plus matching proof evidence.

## Why proofpack is downstream of gate decision

The preserved order is:

```text
gate outcome -> proofpack -> acceptance claim
```

Proof requirements require the future proofpack to link the gate outcome, which means proofpack cannot be a prerequisite for gate readiness.

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

- proof requirements require contract ref;
- proof requirements require run receipt ref;
- proof requirements require gate outcome ref;
- proof requirements can require eval report ref/hash;
- proof requirements can require output artifact refs/hashes;
- missing contract proof target is blocking;
- missing run receipt proof target is blocking;
- missing gate outcome proof target is blocking;
- unsupported proof target is blocking;
- duplicate proof targets do not create conflicting coverage;
- proof requirements do not create proofpack;
- proof requirements do not write `.punk/proofs`;
- proof requirements do not compute artifact hashes from filesystem;
- proof requirements do not write gate outcomes;
- proof requirements do not create acceptance claims;
- proof requirements do not invoke Writer;
- proofpack is not required before gate outcome;
- `ContractStatus` still excludes `accepted`, `rejected`, and `partially_accepted`.

## Checks run

- `python3 scripts/check_research_gate.py` — pass.
- `python3 scripts/check_work_ledger.py` — pass; selected next is `work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md`.
- `cargo fmt --check` — pass.
- `cargo check --workspace` — pass.
- `cargo test -p punk-contract -p punk-eval` — pass, 39 `punk-contract` tests and 6 `punk-eval` tests.
- `git diff --check` — pass.
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md docs/product/CONTRACT-SCHEMA.md work/goals/goal_integrate_contract_proof_requirements_v0_1.md work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md work/STATUS.md work/reports/2026-05-01-contract-proof-requirements-v0-1.md --report work/reports/2026-05-01-contract-proof-requirements-v0-1.md` — pass with 2 existing duplicate-definition warnings in `docs/product/CRATE-STATUS.md`.

## Drift observed

- Existing working tree contains many uncommitted prior goal/report/doc changes. This task used the local uncommitted tree as source of truth and did not attempt to revert unrelated work.
- Existing docs-governance duplicate-definition warnings in `docs/product/CRATE-STATUS.md` may remain until a bounded docs-governance cleanup.
- Engram persistent memory transport returned `Transport closed`; memory save/session summary may fail until the transport is restored.

## Deferred work

- Review checkpoint over the accumulated side-effect-free contract-core model.
- Runtime gate decision writer.
- Runtime proofpack writer and `.punk/proofs` storage.
- Artifact hashing runtime for proof requirements.
- Acceptance claim writer.
- Writer activation.
- Runtime validator execution.
- Provider adapters, policy engine, PKI/signatures, and remote transparency logs.

## Next recommended selected goal

`work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md`

Reason:

After contract draft confirmation, hard-clause mapping, receipt requirements, gate input policy, and proof requirements are modeled, the accumulated contract-core trust seams should be reviewed before selecting another implementation or any Writer/runtime step.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Integrate side-effect-free contract proof requirements and preserve proofpack as downstream of gate outcome."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - docs/product/CONTRACT-SCHEMA.md
    - work/goals/goal_integrate_contract_proof_requirements_v0_1.md
    - work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md
    - work/STATUS.md
    - work/reports/2026-05-01-contract-proof-requirements-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - docs/product/CONTRACT-SCHEMA.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Covered by punk-eval smoke coverage in this task."
```
