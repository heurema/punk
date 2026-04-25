---
id: report_2026_04_25_punk_init_docs_cli_mismatch
goal_id: goal_reconcile_punk_init_docs_cli_mismatch
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Reconcile the `punk init` docs/CLI mismatch without implementing `punk init` or widening the active CLI surface.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a docs/product active-surface reconciliation grounded in current canonical docs and observed CLI behavior. No Deep Research was required.

Research refs:

- `docs/product/RESEARCH-GATE.md`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/DOGFOODING.md`
- `docs/product/FLOW.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/reports/2026-04-25-seventh-work-ledger-review.md`

Decision:
Proceed with docs/work-ledger artifacts only.

## Changed files

- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/DOGFOODING.md`
- `docs/product/FLOW.md`
- `docs/product/CRATE-STATUS.md`
- `work/goals/goal_reconcile_punk_init_docs_cli_mismatch.md`
- `work/goals/goal_run_eighth_work_ledger_review.md`
- `work/reports/2026-04-25-punk-init-docs-cli-mismatch.md`
- `work/STATUS.md`

## What changed

- Documented the implemented CLI surface explicitly:
  - `punk flow inspect`
  - `punk eval run smoke`
  - `punk eval run smoke --format json`
- Removed `punk init` from current active CLI wording in `docs/product/START-HERE.md`.
- Marked `punk init` as a future setup target in `README.md`, `docs/product/START-HERE.md`, and `docs/product/CRATE-STATUS.md`.
- Split `docs/product/DOGFOODING.md` Level 1 command wording into currently implemented subset and target surfaces after implementation.
- Updated `docs/product/FLOW.md` so its command list is a target command list, not current behavior.
- Closed the selected docs/CLI mismatch goal with report refs.
- Added the eighth advisory Work Ledger Review as the next ready goal.
- Updated `work/STATUS.md` so the single live ledger selects the eighth review.

## Active CLI truth

Current implemented CLI surface:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

`punk init` remains deferred.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no CLI command added;
- no `punk init` implementation added;
- no schema changed;
- no `.punk` runtime state written;
- no runtime setup behavior added;
- no gate/proof/proofpack behavior added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Docs now separate implemented CLI behavior from future setup targets."
  touched_surfaces:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/DOGFOODING.md
    - docs/product/FLOW.md
    - docs/product/CRATE-STATUS.md
    - work/goals/goal_reconcile_punk_init_docs_cli_mismatch.md
    - work/goals/goal_run_eighth_work_ledger_review.md
    - work/reports/2026-04-25-punk-init-docs-cli-mismatch.md
    - work/STATUS.md
  required_updates:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/DOGFOODING.md
    - docs/product/FLOW.md
    - docs/product/CRATE-STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Future docs/CLI checks should fail if unimplemented commands are described as active behavior."
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_run_eighth_work_ledger_review.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_run_eighth_work_ledger_review.md`; goals checked: 57.
- `scripts/check.sh docs-governance --files README.md docs/product/START-HERE.md docs/product/DOGFOODING.md docs/product/FLOW.md docs/product/CRATE-STATUS.md work/goals/goal_reconcile_punk_init_docs_cli_mismatch.md work/goals/goal_run_eighth_work_ledger_review.md work/reports/2026-04-25-punk-init-docs-cli-mismatch.md work/STATUS.md --report work/reports/2026-04-25-punk-init-docs-cli-mismatch.md` - PASS; changed files: 9; canonical docs checked: 4; reports checked: 1; failures: 0; warnings: 2 duplicate-definition candidates.
- `cargo test --workspace` - PASS; Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

## Open follow-ups

- Run the eighth advisory Work Ledger Review.
- Implement possible future `punk init` only through a later bounded runtime setup goal.
- Implement runtime gate/proof only through later bounded goals.
- Implement receipt schema/runtime only through a later bounded goal.
- Implement real `.punk` runtime storage only through a later bounded goal.
