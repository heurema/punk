---
id: report_2026_04_25_active_cli_surface_docs_governance_check
goal_id: goal_add_active_cli_surface_docs_governance_check
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add a docs-governance regression guard so current/active Punk CLI docs cannot describe unimplemented commands as active behavior.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded docs-governance check grounded in the just-repaired `punk init` docs/CLI mismatch and current implemented CLI truth. No Deep Research was required.

Research refs:

- `docs/product/RESEARCH-GATE.md`
- `docs/product/START-HERE.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/DOGFOODING.md`
- `docs/product/FLOW.md`
- `work/reports/2026-04-25-punk-init-docs-cli-mismatch.md`
- `work/reports/2026-04-25-eighth-work-ledger-review.md`

Decision:
Proceed with a governance/eval guardrail only.

## Changed files

- `scripts/check_docs_governance.py`
- `docs/product/DOGFOODING.md`
- `evals/cases/docs/README.md`
- `evals/cases/docs/active-cli-surface/README.md`
- `evals/cases/docs/active-cli-surface/unimplemented-command-active.fail.yaml`
- `evals/cases/docs/active-cli-surface/unimplemented-command-imperative.fail.yaml`
- `evals/cases/docs/active-cli-surface/future-command-labeled.pass.yaml`
- `work/goals/goal_add_active_cli_surface_docs_governance_check.md`
- `work/goals/goal_run_ninth_work_ledger_review.md`
- `work/reports/2026-04-25-active-cli-surface-docs-governance-check.md`
- `work/STATUS.md`

## What changed

- Added `DOC_ACTIVE_CLI_UNIMPLEMENTED_COMMAND` to `scripts/check_docs_governance.py`.
- The check applies to active CLI docs surfaces:
  - `README.md`
  - `docs/product/**`
- The implemented CLI allowlist remains exactly:
  - `punk flow inspect`
  - `punk eval run smoke`
  - `punk eval run smoke --format json`
- Future, target, deferred, or not-current command mentions remain allowed when clearly labeled.
- Tightened `docs/product/DOGFOODING.md` Level 2 wording from bare `punk eval` to the implemented `punk eval run smoke` subset.
- Added fixture-style docs eval cases for active CLI surface truth.
- Added the ninth advisory Work Ledger Review as the next ready goal.

## Regression behavior

Hard-fail examples:

- current implemented CLI list contains `punk init`;
- current user-flow instructions say to run `punk init`;
- active/current wording presents any Punk command outside the implemented allowlist as working behavior.

Allowed examples:

- `punk init` appears under future target commands;
- future gate/proof commands appear under future/deferred/target wording;
- docs explicitly say a command is not current behavior.

## Active CLI truth

Current implemented CLI surface:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

Everything else remains future/deferred until a later bounded implementation goal changes the allowlist and the actual CLI behavior together.

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
  reason: "Docs-governance now enforces active CLI surface honesty, and DOGFOODING wording was aligned to the implemented eval command."
  touched_surfaces:
    - scripts/check_docs_governance.py
    - docs/product/DOGFOODING.md
    - evals/cases/docs/README.md
    - evals/cases/docs/active-cli-surface/README.md
    - evals/cases/docs/active-cli-surface/unimplemented-command-active.fail.yaml
    - evals/cases/docs/active-cli-surface/unimplemented-command-imperative.fail.yaml
    - evals/cases/docs/active-cli-surface/future-command-labeled.pass.yaml
    - work/goals/goal_add_active_cli_surface_docs_governance_check.md
    - work/goals/goal_run_ninth_work_ledger_review.md
    - work/reports/2026-04-25-active-cli-surface-docs-governance-check.md
    - work/STATUS.md
  required_updates:
    - docs/product/DOGFOODING.md
    - evals/cases/docs/README.md
    - evals/cases/docs/active-cli-surface/README.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Fixture cases added under evals/cases/docs/active-cli-surface/."
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files scripts/check_docs_governance.py docs/product/DOGFOODING.md evals/cases/docs/README.md evals/cases/docs/active-cli-surface/README.md evals/cases/docs/active-cli-surface/unimplemented-command-active.fail.yaml evals/cases/docs/active-cli-surface/unimplemented-command-imperative.fail.yaml evals/cases/docs/active-cli-surface/future-command-labeled.pass.yaml work/goals/goal_add_active_cli_surface_docs_governance_check.md work/goals/goal_run_ninth_work_ledger_review.md work/reports/2026-04-25-active-cli-surface-docs-governance-check.md work/STATUS.md --report work/reports/2026-04-25-active-cli-surface-docs-governance-check.md` - PASS
- `python3 scripts/check_docs_governance.py --files README.md docs/product/START-HERE.md docs/product/CRATE-STATUS.md docs/product/DOGFOODING.md docs/product/FLOW.md --report work/reports/2026-04-25-active-cli-surface-docs-governance-check.md` - PASS, with 2 existing duplicate-definition candidate warnings unrelated to active CLI surface truth
- temp fixture with active `punk init` in current implemented CLI list - PASS, expected failure observed with `DOC_ACTIVE_CLI_UNIMPLEMENTED_COMMAND`
- temp fixture with future `punk init` / gate commands under future target wording - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found

## Open follow-ups

- Run the ninth advisory Work Ledger Review.
- If a future goal implements new CLI behavior, update both the CLI implementation and the docs-governance implemented-command allowlist in the same bounded change.
- Implement possible future `punk init` only through a later bounded runtime setup goal.
- Keep runtime gate/proof, receipt schema/runtime, proofpack writer, and `.punk` storage work deferred until separately selected.
