---
id: report_2026_04_25_run_receipt_kernel_minimal_fields_v0_1
goal_id: goal_extend_run_receipt_kernel_minimal_fields_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Extend the side-effect-free run receipt kernel with minimal fields v0.1 so later gate/proof work has a stronger evidence shape.

## Research Gate

Classification: R0
Required: no
Rationale:
This implements already-defined receipt field, missing-validator, and proof-before-acceptance boundaries in the existing Rust receipt kernel. No new architecture decision or external research was required.

Research refs:

- `evals/specs/minimal-receipt-fields.v0.1.md`
- `evals/specs/missing-validator-policy.v0.1.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `work/reports/2026-04-25-ninth-work-ledger-review.md`

Decision:
Proceed with a side-effect-free kernel implementation only.

## Changed files

- `crates/punk-domain/src/lib.rs`
- `crates/punk-flow/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md`
- `work/goals/goal_run_tenth_work_ledger_review.md`
- `work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `RUN_RECEIPT_SCHEMA_VERSION` and explicit `ProducedAt` to `RunReceipt`.
- Added optional receipt context refs:
  - `work_goal_ref`
  - `flow_state_before`
  - `flow_state_after`
- Added `BoundaryNote` support so receipt caveats remain explicit.
- Added missing-validator vocabulary to the receipt kernel:
  - `ValidatorRequiredness`: `required`, `advisory`, `conditional`
  - `ValidatorOutcome`: `passed`, `failed`, `unavailable`, `skipped`, `unsupported`, `deferred`
  - `ValidatorEvidence` entries with validator identity, kind, scope, reason, summary, evidence ref, observed time, and follow-up ref.
- Added helpers that keep evidence gaps visible:
  - `is_required_missing_evidence_gap`
  - `blocks_proof_claim`
  - `has_required_validator_evidence_gaps`
  - `has_required_validator_blockers`
- Updated flow/eval receipt call sites to pass explicit `ProducedAt`.
- Added tests for identity fields, required missing-validator gaps, failed validators, non-passing reason codes, and receipt boundary preservation.
- Added the tenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

The receipt remains evidence-only.

It still does not:

- imply final acceptance;
- write gate decisions;
- create proofpacks;
- require runtime storage;
- create receipts for denied transitions;
- hide missing-validator evidence gaps.

## Active CLI truth

No product CLI behavior changed.

Current implemented CLI surface remains:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

## Scope boundaries preserved

- no `.punk` runtime state written;
- no runtime storage implemented;
- no `Cargo.toml` or `Cargo.lock` changed;
- no CLI command added;
- no `punk init` implementation added;
- no schema file added;
- no gate/proof/proofpack behavior added;
- no semantic assessor implementation added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Active-core receipt kernel behavior changed, and the work ledger/report record the change; product docs and specs already covered the boundary."
  touched_surfaces:
    - crates/punk-domain/src/lib.rs
    - crates/punk-flow/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md
    - work/goals/goal_run_tenth_work_ledger_review.md
    - work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Rust unit coverage added/updated in `crates/punk-domain`, `crates/punk-flow`, and `crates/punk-eval` call sites."
```

## Checks run

- `cargo fmt --all --check` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files crates/punk-domain/src/lib.rs crates/punk-flow/src/lib.rs crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md work/goals/goal_run_tenth_work_ledger_review.md work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md --report work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md` - PASS
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `cargo run -p punk-cli -- eval run smoke --format json` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found

## Open follow-ups

- Run the tenth advisory Work Ledger Review.
- Decide whether the next implementation branch should be gate decision kernel, proofpack kernel, runtime storage, or another receipt/eval step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
- If receipt serialization/schema files are later needed, add them through a separate bounded schema goal.
