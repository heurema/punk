---
id: report_2026_04_25_missing_validator_policy_v0_1
goal_id: goal_define_missing_validator_policy_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Define missing-validator policy v0.1 as a docs/spec-only artifact before runtime gate/proof/storage, receipt expansion, or semantic assessor interface work.

## Research Gate

Classification: R1
Required: yes
Rationale:
The policy is grounded in existing executor-agnostic validation, run receipt, gate decision, proofpack, and Project Memory storage boundary artifacts. No new Deep Research was required.

Research refs:

- `docs/product/RESEARCH-GATE.md`
- `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/project-memory-storage-boundary.v0.1.md`
- `work/reports/2026-04-25-fifth-work-ledger-review.md`

Decision:
Proceed with docs/spec/work-ledger artifacts only.

## Changed files

- `evals/specs/missing-validator-policy.v0.1.md`
- `evals/specs/executor-agnostic-validation-boundary.v0.1.md`
- `work/goals/goal_define_missing_validator_policy_v0_1.md`
- `work/goals/goal_define_minimal_receipt_fields_v0_1.md`
- `work/reports/2026-04-25-missing-validator-policy-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `evals/specs/missing-validator-policy.v0.1.md`.
- Linked missing-validator policy from the executor-agnostic validation boundary eval spec.
- Closed the selected missing-validator policy goal with report/spec refs.
- Added the minimal receipt fields v0.1 goal as the next ready goal.
- Updated `work/STATUS.md` so the single live ledger selects the minimal receipt fields goal.

## Policy summary

Shorthand:

```text
missing is visible
missing is not passing
missing is not proof
```

The policy distinguishes:

- `passed` - validator ran and checked scope passed;
- `failed` - validator ran and found a violation;
- `unavailable` - tool, dependency, service, credential, file, or runtime was missing;
- `skipped` - intentionally not run, with reason;
- `unsupported` - not applicable to current project/platform/artifact/scope, with reason;
- `deferred` - intentionally left unresolved with follow-up context.

Validator outcomes are evidence inputs.

They are not final decisions.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no schema changed;
- no `.punk` runtime state written;
- no validator implementation added;
- no runtime gate/proof behavior added;
- no CLI command added;
- no `punk init` command added;
- no proofpack writer added;
- no semantic assessor interface added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Missing-validator policy v0.1 defines validation evidence-gap semantics before runtime gate/proof/storage implementation."
  touched_surfaces:
    - evals/specs/missing-validator-policy.v0.1.md
    - evals/specs/executor-agnostic-validation-boundary.v0.1.md
    - work/goals/goal_define_missing_validator_policy_v0_1.md
    - work/goals/goal_define_minimal_receipt_fields_v0_1.md
    - work/reports/2026-04-25-missing-validator-policy-v0-1.md
    - work/STATUS.md
  required_updates:
    - evals/specs/missing-validator-policy.v0.1.md
    - evals/specs/executor-agnostic-validation-boundary.v0.1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Future implementation must keep missing-validator outcomes visible and non-authoritative."
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_define_minimal_receipt_fields_v0_1.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_define_minimal_receipt_fields_v0_1.md`; goals checked: 51.
- `scripts/check.sh docs-governance --files evals/specs/missing-validator-policy.v0.1.md evals/specs/executor-agnostic-validation-boundary.v0.1.md work/goals/goal_define_missing_validator_policy_v0_1.md work/goals/goal_define_minimal_receipt_fields_v0_1.md work/reports/2026-04-25-missing-validator-policy-v0-1.md work/STATUS.md --report work/reports/2026-04-25-missing-validator-policy-v0-1.md` - PASS; changed files: 6; canonical docs checked: 0; reports checked: 1; failures: 0; warnings: 0.
- `cargo test --workspace` - PASS; 53 Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

## Open follow-ups

- Define minimal receipt fields v0.1.
- Define semantic assessor command interface.
- Implement real gate/proofpack runtime only through later bounded goals.
- Implement real `.punk` runtime storage only through later bounded goals.
- Revisit possible future `punk init` only when runtime setup is intentionally scoped.
