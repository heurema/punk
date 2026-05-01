---
id: report_2026_04_30_hard_clause_mapping_v0_1
kind: work-report
status: final
authority: evidence
created_at: 2026-04-30
updated_at: 2026-04-30
goal_ref: work/goals/goal_integrate_hard_clause_mapping_v0_1.md
research_gate: R1
external_research: not-used
---

# Hard Clause Mapping v0.1

## Summary

Implemented a side-effect-free hard-clause mapping boundary for Punk contracts.

Every hard clause must now have at least one valid declarative enforcement or review path before `approved_for_run` model state:

- validator ref;
- required receipt field;
- proof requirement ref;
- explicit human gate review with a non-empty reason.

Unsupported hard clauses are not valid mappings. They are surfaced as blocking findings and prevent `approved_for_run` until fixed.

## Files changed

- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CONTRACT-SCHEMA.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_integrate_hard_clause_mapping_v0_1.md`
- `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md`
- `work/reports/2026-04-30-hard-clause-mapping-v0-1.md`

## Mapping model added

Added side-effect-free hard-clause mapping model concepts in `punk-contract`:

- `HardClauseMappingTarget`
- `HardClauseMappingStatus`
- `HardClauseMappingFinding`
- `HardClauseMappingAssessment`
- `assess_hard_clause_mappings`

The existing contract draft confirmation boundary now consults hard-clause mapping before producing `approved_for_run` model state.

## Valid hard-clause mapping

A hard clause is valid only when it maps to at least one of:

- `validator_ref`
- `required_receipt_field`
- `proof_requirement_ref`
- `human_gate_review` with a non-empty reason

A blank human gate review reason is invalid.

## Why unsupported hard clauses block approval

Unsupported hard clauses are visible findings, not valid enforcement paths.

This prevents the loophole:

```text
hard clause has no enforcement path
-> mark unsupported
-> approve anyway
```

Instead, unsupported hard clauses block `approved_for_run` until mapped, downgraded, removed, or converted into clarification/assumption handling in a later bounded step.

## Soft/advisory clauses

Soft and advisory clauses may remain unmapped. They do not block approval merely because they lack validator, receipt, proof, or human review mappings.

They also cannot override hard-clause blockers.

## Approval consultation

`confirm_contract_draft` now blocks approval when hard-clause mapping assessment has blocking findings:

- unsupported hard clause;
- hard clause without mapping target;
- hard rationale clause;
- blank human gate review reason.

Mapped hard clauses are preserved in the approved-for-run model state with their mapping assessment.

## No runtime validator execution

Validator refs remain references only. The model does not execute validators.

## No gate/proof writing

Human gate review is a declared future gate input only. It does not write a gate outcome.

Proof requirement refs are declarative only. They do not create proofpacks.

## Writer boundary

Hard-clause mapping does not invoke, select, depend on, or authorize Writer.

Writer remains downstream of:

```text
approved contract
-> run receipt/evidence
-> gate decision
-> proofpack
```

## Eval coverage added

Added smoke coverage for:

- hard clause with validator ref is mapped;
- hard clause with required receipt field is mapped;
- hard clause with proof requirement ref is mapped;
- hard clause with explicit human gate review and reason is mapped;
- blank human review reason blocks;
- hard clause with no mapping is `unsupported_blocking`;
- unsupported hard clause blocks `approved_for_run`;
- soft/advisory clauses without mapping do not block;
- hard rationale clause is invalid/blocking;
- confirmed ready draft with all hard clauses mapped can become `approved_for_run`;
- confirmed ready draft with unmapped hard clause cannot become `approved_for_run`;
- mapping does not write gate outcomes;
- mapping does not create proofpacks;
- mapping does not invoke Writer;
- `ContractStatus` still excludes acceptance outcomes.

## Checks run

- `git status --short --branch` — PASS, with pre-existing dirty/untracked working tree entries still present.
- `python3 scripts/check_research_gate.py` — PASS.
- `python3 scripts/check_work_ledger.py` — PASS.
- `cargo fmt --all` — PASS.
- `cargo fmt --check` — PASS.
- `cargo check --workspace` — PASS.
- `cargo test -p punk-contract -p punk-eval` — PASS.
- `git diff --check` — PASS.
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CONTRACT-SCHEMA.md docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_integrate_hard_clause_mapping_v0_1.md work/goals/goal_integrate_contract_receipt_requirements_v0_1.md work/reports/2026-04-30-hard-clause-mapping-v0-1.md --report work/reports/2026-04-30-hard-clause-mapping-v0-1.md` — PASS with 0 failures and 2 scoped warnings.

## Drift observed

No new runtime/CLI/Writer/gate/proof drift was introduced.

Remaining docs-governance warnings are low-severity documentation-heading warnings and are tracked in `work/STATUS.md`.

## Deferred work

- Runtime validator execution.
- Runtime contract storage.
- Runtime receipt storage.
- Gate decision writer.
- Proofpack writer.
- Writer activation.
- Policy engine integration.

## Next recommended selected goal

Selected next:

```text
work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
```

Reason: after hard clauses have explicit enforcement/review paths, the next safe seam is connecting contract receipt requirements to run receipt models.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Added side-effect-free hard clause mapping model/eval behavior and clarified that unsupported hard clauses block approval."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CONTRACT-SCHEMA.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_integrate_hard_clause_mapping_v0_1.md
    - work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
    - work/reports/2026-04-30-hard-clause-mapping-v0-1.md
  required_updates:
    - docs/product/CONTRACT-SCHEMA.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_integrate_hard_clause_mapping_v0_1.md
    - work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test -p punk-contract -p punk-eval
    - cargo check --workspace
```
