---
id: report_2026_04_25_minimal_receipt_fields_v0_1
goal_id: goal_define_minimal_receipt_fields_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Define minimal receipt fields v0.1 as a docs/spec-only artifact before receipt schema/runtime work.

## Research Gate

Classification: R1
Required: yes
Rationale:
The receipt fields spec is grounded in the run receipt boundary and missing-validator policy. No new Deep Research was required.

Research refs:

- `docs/product/RESEARCH-GATE.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/missing-validator-policy.v0.1.md`
- `work/reports/2026-04-25-missing-validator-policy-v0-1.md`

Decision:
Proceed with docs/spec/work-ledger artifacts only.

## Changed files

- `evals/specs/minimal-receipt-fields.v0.1.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/missing-validator-policy.v0.1.md`
- `work/goals/goal_define_minimal_receipt_fields_v0_1.md`
- `work/goals/goal_define_semantic_assessor_command_interface_v0_1.md`
- `work/reports/2026-04-25-minimal-receipt-fields-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `evals/specs/minimal-receipt-fields.v0.1.md`.
- Linked the field spec from run receipt and missing-validator specs.
- Closed the selected minimal receipt fields goal with report/spec refs.
- Added semantic assessor command interface v0.1 as the next ready goal.
- Updated `work/STATUS.md` so the single live ledger selects the semantic assessor interface goal.

## Field summary

A future receipt should carry enough evidence context for:

- receipt identity;
- contract and run refs;
- run scope refs;
- event and eval refs;
- validator outcomes;
- missing-validator evidence gaps;
- produced artifact refs;
- runner/command refs where stable;
- boundary notes.

Receipt fields are evidence fields.

They do not decide.

```text
receipt = what a bounded run did and observed
gate = future closure authority
proofpack = future provenance bundle after gate
```

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no schema changed;
- no `.punk` runtime state written;
- no receipt schema implementation added;
- no validator implementation added;
- no runtime gate/proof behavior added;
- no CLI command added;
- no `punk init` command added;
- no proofpack writer added;
- no semantic assessor interface implementation added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Minimal receipt fields v0.1 defines evidence field boundaries before receipt schema/runtime implementation."
  touched_surfaces:
    - evals/specs/minimal-receipt-fields.v0.1.md
    - evals/specs/run-receipt-boundary.v0.1.md
    - evals/specs/missing-validator-policy.v0.1.md
    - work/goals/goal_define_minimal_receipt_fields_v0_1.md
    - work/goals/goal_define_semantic_assessor_command_interface_v0_1.md
    - work/reports/2026-04-25-minimal-receipt-fields-v0-1.md
    - work/STATUS.md
  required_updates:
    - evals/specs/minimal-receipt-fields.v0.1.md
    - evals/specs/run-receipt-boundary.v0.1.md
    - evals/specs/missing-validator-policy.v0.1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Future receipt implementation must keep receipts as evidence, not gate decisions or proofpacks."
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_define_semantic_assessor_command_interface_v0_1.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_define_semantic_assessor_command_interface_v0_1.md`; goals checked: 52.
- `scripts/check.sh docs-governance --files evals/specs/minimal-receipt-fields.v0.1.md evals/specs/run-receipt-boundary.v0.1.md evals/specs/missing-validator-policy.v0.1.md work/goals/goal_define_minimal_receipt_fields_v0_1.md work/goals/goal_define_semantic_assessor_command_interface_v0_1.md work/reports/2026-04-25-minimal-receipt-fields-v0-1.md work/STATUS.md --report work/reports/2026-04-25-minimal-receipt-fields-v0-1.md` - PASS; changed files: 7; canonical docs checked: 0; reports checked: 1; failures: 0; warnings: 0.
- `cargo test --workspace` - PASS; 53 Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

## Open follow-ups

- Define semantic assessor command interface v0.1.
- Implement receipt schema/runtime only through a later bounded goal.
- Implement real gate/proofpack runtime only through later bounded goals.
- Implement real `.punk` runtime storage only through later bounded goals.
- Revisit possible future `punk init` only when runtime setup is intentionally scoped.
